//! Publish npm package metadata and attachment tarballs.
//! Publish metadata package npm và tarball attachment.

use super::super::super::scope::scoped_full;
use crate::{AppState, model::Package};
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use std::collections::HashMap;
use tracing::warn;

pub(in crate::npm) async fn publish_package_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Path((scope, name)): Path<(String, String)>,
    body: Bytes,
) -> Result<Json<Package>, StatusCode> {
    let result = publish_package(
        State(state),
        headers,
        Path(scoped_full(&scope, &name)),
        peer,
        body,
    )
    .await
    .inspect_err(|e| eprintln!("publish_package_scoped error: {e:?}"));
    eprintln!(
        "publish_package_scoped: scope={} name={} result={:?}",
        scope,
        name,
        result.as_ref().map(|p| p.name.clone())
    );
    result
}

pub(in crate::npm) async fn publish_package(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    Path(name): Path<String>,
    peer: crate::OptionalPeerAddress,
    body: Bytes,
) -> Result<Json<Package>, StatusCode> {
    crate::trusted::scoped_package("npm", &name)?;
    let trusted_identity = auth
        .authorize_package_write_from(&headers, &name, crate::peer_address(peer))
        .await?;
    // npm CLI thật gửi metadata + _attachments (tarball base64) trong 1 PUT
    use base64::Engine;
    use sha2::{Digest, Sha512};

    let mut doc: serde_json::Value =
        serde_json::from_slice(&body).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Lưu tarball từ _attachments → blob content-addressed, gắn integrity vào dist.
    // Fail-closed: attachment sai (thiếu data / base64 hỏng) → từ chối, không publish "mù"
    let mut attachments = Vec::new();
    if let Some(atts) = doc.get("_attachments").and_then(|a| a.as_object()) {
        for (filename, att) in atts {
            let data = att
                .get("data")
                .and_then(|d| d.as_str())
                .ok_or(StatusCode::BAD_REQUEST)?;
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(data)
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            attachments.push((filename.clone(), bytes));
        }
    }
    if attachments.len() > 1 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if let Some((_attachment_filename, blob)) = attachments.pop() {
        let versions = doc
            .get("versions")
            .and_then(|versions| versions.as_object())
            .ok_or(StatusCode::BAD_REQUEST)?;
        if versions.len() != 1 || blob.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }
        let mut hasher = Sha512::new();
        hasher.update(&blob);
        let b64 = base64::engine::general_purpose::STANDARD.encode(hasher.finalize());
        let digest = format!("sha512-{b64}");
        store
            .put_blob(&digest, &blob)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        // Bind the single attachment to the single version and registry origin.
        // Gắn attachment duy nhất cho version duy nhất và origin của registry.
        let host = headers
            .get("host")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("localhost");
        let versions = doc
            .get_mut("versions")
            .and_then(|versions| versions.as_object_mut())
            .ok_or(StatusCode::BAD_REQUEST)?;
        let (version_key, version) = versions.iter_mut().next().ok_or(StatusCode::BAD_REQUEST)?;
        let version_key = version_key.clone();
        if let Some(dist) = version.get_mut("dist") {
            dist["integrity"] = serde_json::Value::String(digest);
            let unscoped_name = name.rsplit('/').next().unwrap_or(&name);
            let filename = format!("{unscoped_name}-{version_key}.tgz");
            dist["tarball"] =
                serde_json::Value::String(format!("http://{host}/{}/-/{filename}", name));
        } else {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let pkg: Package = serde_json::from_value(doc).map_err(|e| {
        warn!("publish {}: body parse fail: {e}", name);
        StatusCode::BAD_REQUEST
    })?;

    // Verify name matches
    if pkg.name != name {
        return Err(StatusCode::BAD_REQUEST);
    }

    if let Some((identity, binding_generation, sigstore_oidc_token)) = trusted_identity {
        if pkg.versions.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }
        for version in pkg.versions.values() {
            if !crate::trusted::is_valid_sha512_integrity(&version.dist.integrity) {
                return Err(StatusCode::BAD_REQUEST);
            }
            if store
                .get_blob(&version.dist.integrity)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .is_none()
            {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
        let key = auth.attestation_key().ok_or(StatusCode::NOT_FOUND)?;
        let mut public_attestations = HashMap::new();
        for (version, package_version) in &pkg.versions {
            let blob = store
                .get_blob(&package_version.dist.integrity)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::BAD_REQUEST)?;
            let artifact_sha256 = hex::encode(sha2::Sha256::digest(&blob));
            let bundle = crate::trusted::sign_sigstore_attestation(
                &sigstore_oidc_token,
                &identity,
                &pkg.name,
                version,
                &format!("{}@{}", pkg.name, version),
                &package_version.dist.integrity,
                &artifact_sha256,
            )
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
            public_attestations.insert(version.clone(), (artifact_sha256, bundle));
        }
        store
            .put_trusted_package(
                &pkg,
                &identity,
                key,
                binding_generation,
                &public_attestations,
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        store
            .put_package(&pkg)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    let version = pkg
        .versions
        .keys()
        .next()
        .cloned()
        .or_else(|| pkg.dist_tags.get("latest").cloned());
    let _ = store
        .audit("publish", &pkg.name, version.as_deref(), None)
        .await;
    Ok(Json(pkg))
}

use super::super::prelude::{Bytes, IntoResponse, RegistryStore, Response, StatusCode};
use super::super::{
    manifest_validation::{validate_oci_manifest, validate_oci_reference},
    router::header_value,
};
use sha2::{Digest, Sha256};

pub(in crate::oci::oci) async fn manifest_get(
    store: &RegistryStore,
    name: &str,
    reference: &str,
) -> Result<Response, StatusCode> {
    match store.get_oci_manifest_raw(name, reference).await {
        Ok(Some((manifest_json, digest))) => {
            let media_type = serde_json::from_str::<serde_json::Value>(&manifest_json)
                .ok()
                .and_then(|manifest| {
                    manifest
                        .get("mediaType")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned)
                })
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
            let mut resp = Response::new(axum::body::Body::from(manifest_json));
            resp.headers_mut()
                .insert(axum::http::header::CONTENT_TYPE, header_value(&media_type)?);
            // Add Docker-Content-Digest (stored digest of the exact bytes pushed)
            resp.headers_mut()
                .insert("docker-content-digest", header_value(&digest)?);
            Ok(resp.into_response())
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub(in crate::oci::oci) async fn manifest_put(
    store: &RegistryStore,
    name: &str,
    reference: &str,
    body: Bytes,
    trusted_publisher: Option<(mgc_oidc::OidcClaims, u64, String)>,
    attestation_key: Option<&mgc_crypto::KeyPair>,
) -> Result<Response, StatusCode> {
    validate_oci_reference(reference)?;
    let manifest_digest = format!("sha256:{}", hex::encode(Sha256::digest(&body)));
    if reference.starts_with("sha256:") && reference != manifest_digest {
        return Err(StatusCode::BAD_REQUEST);
    }
    let manifest_location = format!("/v2/{name}/manifests/{reference}");
    let location_header = header_value(&manifest_location)?;
    let digest_header = header_value(&manifest_digest)?;
    validate_oci_manifest(store, name, &body).await?;

    if let Some((identity, binding_generation, sigstore_oidc_token)) = trusted_publisher {
        let package_scope = crate::trusted::scoped_package("oci", name)?;
        let artifact_sha256 = hex::encode(Sha256::digest(&body));
        let registry_digest = format!("sha256:{artifact_sha256}");
        let public_bundle = crate::trusted::sign_sigstore_attestation(
            &sigstore_oidc_token,
            &identity,
            &package_scope,
            reference,
            &format!("{name}@{reference}"),
            &registry_digest,
            &artifact_sha256,
        )
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
        let key = attestation_key.ok_or(StatusCode::NOT_FOUND)?;
        store
            .put_trusted_oci_manifest(crate::storage::TrustedOciManifestInput {
                repo: name,
                reference,
                manifest_bytes: &body,
                attestation: crate::storage::TrustedAttestationInput {
                    package: &package_scope,
                    version: reference,
                    registry_digest: &registry_digest,
                    artifact_sha256: &artifact_sha256,
                    claims: &identity,
                    key,
                    expected_binding_generation: binding_generation,
                    sigstore_bundle: public_bundle,
                },
            })
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        store
            .put_oci_manifest(name, reference, &body)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    let mut response = StatusCode::CREATED.into_response();
    response.headers_mut().insert("location", location_header);
    response
        .headers_mut()
        .insert("docker-content-digest", digest_header);
    Ok(response)
}

pub(in crate::oci::oci) async fn manifest_delete(
    store: &RegistryStore,
    name: &str,
    reference: &str,
) -> Result<Response, StatusCode> {
    store
        .delete_oci_manifest(name, reference)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

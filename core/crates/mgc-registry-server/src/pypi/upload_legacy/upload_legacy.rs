//! Handles the Twine-compatible legacy upload endpoint.
//! Xử lý endpoint upload legacy tương thích Twine.

use super::error::UploadLegacyError;
use crate::{AppState, model::PypiFile};
use axum::{
    extract::{Multipart, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use tracing::warn;

/// Twine-compatible upload: POST multipart /pypi/legacy/
/// fields: :action=file_upload, name, version, filetype, sha256_digest, content (file)
pub(in crate::pypi) async fn upload_legacy(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    mut multipart: Multipart,
) -> Result<Response, UploadLegacyError> {
    // Reject missing credentials before buffering multipart content — từ chối auth thiếu trước khi đọc file.
    if auth.admin_token.is_some() && auth.authenticate_headers_async(&headers).await.is_none() {
        return Err(UploadLegacyError::RequestStatus(StatusCode::UNAUTHORIZED));
    }
    let mut name: Option<String> = None;
    let mut version: Option<String> = None;
    let mut filename: Option<String> = None;
    let mut sha256_digest: Option<String> = None;
    let mut content: Option<Vec<u8>> = None;
    let mut requires_python: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(UploadLegacyError::Multipart)?
    {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "name" => name = field.text().await.ok(),
            "version" => version = field.text().await.ok(),
            "filename" => filename = field.file_name().map(|s| s.to_string()),
            "sha256_digest" => sha256_digest = field.text().await.ok(),
            "requires_python" => requires_python = field.text().await.ok(),
            "content" => {
                filename = field.file_name().map(|s| s.to_string());
                content = Some(
                    field
                        .bytes()
                        .await
                        .map_err(UploadLegacyError::Multipart)?
                        .to_vec(),
                )
            }
            _ => {}
        }
    }

    let name = name
        .filter(|n| !n.is_empty())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let name = crate::trusted::canonical_pypi_name(&name)?;
    let package_scope = crate::trusted::scoped_package("pypi", &name)?;
    let trusted_publisher = auth
        .authorize_package_write_from(&headers, &package_scope, crate::peer_address(peer))
        .await?;
    let version = version
        .filter(|v| !v.is_empty())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let filename = filename
        .filter(|f| !f.is_empty())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let content = content.ok_or(StatusCode::BAD_REQUEST)?;

    // sha256 verify (fail-closed: không đúng digest → từ chối)
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let actual = format!("sha256:{}", hex::encode(hasher.finalize()));
    let expected = match sha256_digest {
        Some(d) => {
            let d = d.trim().to_lowercase();
            if d.starts_with("sha256:") {
                d
            } else {
                format!("sha256:{d}")
            }
        }
        None => actual.clone(),
    };
    if actual != expected {
        warn!(
            "pypi upload {}: sha256 mismatch ({} != {})",
            filename, actual, expected
        );
        return Err(UploadLegacyError::DigestMismatch);
    }

    store
        .put_blob(&actual, &content)
        .await
        .map_err(UploadLegacyError::Storage)?;

    let file = PypiFile {
        name: name.clone(),
        version: version.clone(),
        filename: filename.clone(),
        digest: actual.clone(),
        size: content.len() as i64,
        requires_python,
    };
    if let Some((identity, binding_generation, sigstore_oidc_token)) = trusted_publisher {
        let artifact_sha256 = actual
            .strip_prefix("sha256:")
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let key = auth.attestation_key().ok_or(StatusCode::NOT_FOUND)?;
        let bundle = crate::trusted::sign_sigstore_attestation(
            &sigstore_oidc_token,
            &identity,
            &package_scope,
            &version,
            &format!("{name}/{filename}"),
            &actual,
            artifact_sha256,
        )
        .await
        .map_err(UploadLegacyError::Signing)?;
        store
            .put_trusted_pypi_file(
                &file,
                &identity,
                key,
                binding_generation,
                artifact_sha256,
                bundle,
            )
            .await
            .map_err(UploadLegacyError::Storage)?;
    } else {
        store
            .put_pypi_file(&file)
            .await
            .map_err(UploadLegacyError::Storage)?;
    }

    Ok(JsonOk {
        message: "file uploaded".to_string(),
    }
    .into_response())
}
#[derive(Serialize)]
struct JsonOk {
    message: String,
}

impl IntoResponse for JsonOk {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

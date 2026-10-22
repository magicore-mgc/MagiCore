//! Store uploaded tarballs by their integrity digest.
//! Lưu tarball tải lên theo digest integrity.

use super::super::super::scope::scoped_full;
use crate::AppState;
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};

pub(in crate::npm) async fn upload_tarball_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Path((scope, name, filename)): Path<(String, String, String)>,
    body: Bytes,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = upload_tarball(
        State(state),
        headers,
        peer,
        Path((scoped_full(&scope, &name), filename.clone())),
        body,
    )
    .await;
    eprintln!(
        "upload_tarball_scoped: scope={} name={} filename={} result={:?}",
        scope,
        name,
        filename,
        result.as_ref()
    );
    result
}

pub(in crate::npm) async fn upload_tarball(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Path((_name, _filename)): Path<(String, String)>,
    body: Bytes,
) -> Result<Json<serde_json::Value>, StatusCode> {
    auth.authorize_npm_write_from(&headers, &_name, crate::peer_address(peer))
        .await?;
    use sha2::{Digest, Sha512};

    let mut hasher = Sha512::new();
    hasher.update(&body);
    let b64 = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        hasher.finalize(),
    );
    let digest = format!("sha512-{b64}");
    store
        .put_blob(&digest, &body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let _ = store
        .audit("upload-tarball", &_name, Some(&_filename), None)
        .await;
    Ok(Json(
        serde_json::json!({ "ok": true, "digest": digest, "size": body.len() }),
    ))
}

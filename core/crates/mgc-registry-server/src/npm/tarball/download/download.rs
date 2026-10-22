//! Resolve and serve a stored or verified upstream tarball.
//! Tìm và phục vụ tarball đã lưu hoặc upstream đã xác minh.

use super::super::super::scope::scoped_full;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

pub(in crate::npm) async fn download_tarball_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((scope, name, filename)): Path<(String, String, String)>,
) -> Result<Response, StatusCode> {
    let result = download_tarball(
        State(state),
        headers,
        Path((scoped_full(&scope, &name), filename.clone())),
    )
    .await;
    eprintln!(
        "download_tarball_scoped: scope={} name={} filename={} status={:?}",
        scope,
        name,
        filename,
        result.as_ref().map(|_| "ok")
    );
    result
}

pub(super) fn content_disposition(filename: &str) -> Result<HeaderValue, StatusCode> {
    HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub(in crate::npm) async fn download_tarball(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    Path((name, filename)): Path<(String, String)>,
) -> Result<Response, StatusCode> {
    auth.authorize_package_read_async(&headers, &name).await?;
    // filename: :unscoped-name-:version.tgz → version → dist.integrity → blob
    let unscoped = name.rsplit('/').next().unwrap_or(&name);
    let version = filename
        .strip_suffix(".tgz")
        .and_then(|s| s.strip_prefix(&format!("{}-", unscoped)));
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(version) = version
        && let Some(pkg) = store.get_package(&name).await.ok().flatten()
        && let Some(v) = pkg.versions.get(version)
    {
        let digest = &v.dist.integrity;
        if !digest.is_empty() {
            if let Some(data) = store.get_blob(digest).await.ok().flatten() {
                let mut resp = axum::response::Response::new(axum::body::Body::from(data));
                resp.headers_mut().insert(
                    axum::http::header::CONTENT_TYPE,
                    HeaderValue::from_static("application/octet-stream"),
                );
                resp.headers_mut()
                    .insert("content-disposition", content_disposition(&filename)?);
                return Ok(resp.into_response());
            }
            // ITEM 4: blob miss → proxy tarball từ upstream, cache vào store
            // Upstream bytes are verified against the declared digest BEFORE
            // caching and serving — a mismatched upstream response is dropped
            // (fail-closed) instead of being cached as poison.
            // Bytes từ upstream được đối chiếu digest khai TRƯỚC khi cache và
            // serve — lệch digest thì bỏ (fail-closed), không cache dữ liệu bẩn.
            if let Ok(Some(data)) = store.fetch_upstream_tarball(&v.dist.tarball).await {
                match store.put_blob(digest, &data).await {
                    Ok(()) => {}
                    Err(e) => {
                        tracing::warn!("upstream tarball digest mismatch, not cached: {e:#}");
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
                let mut resp = axum::response::Response::new(axum::body::Body::from(data));
                resp.headers_mut().insert(
                    axum::http::header::CONTENT_TYPE,
                    HeaderValue::from_static("application/octet-stream"),
                );
                resp.headers_mut()
                    .insert("content-disposition", content_disposition(&filename)?);
                return Ok(resp.into_response());
            }
        }
    }

    Err(StatusCode::NOT_FOUND)
}

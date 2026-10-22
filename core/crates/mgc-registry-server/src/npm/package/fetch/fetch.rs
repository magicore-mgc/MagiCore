//! Read npm package metadata and scoped aliases.
//! Đọc metadata package npm và route alias có scope.

use super::super::super::scope::scoped_full;
use crate::{AppState, model::Package};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use tracing::warn;

pub(in crate::npm) async fn get_package_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((scope, name)): Path<(String, String)>,
) -> Result<Json<Package>, StatusCode> {
    let result = get_package(State(state), headers, Path(scoped_full(&scope, &name))).await;
    eprintln!(
        "get_package_scoped: scope={} name={} result={:?}",
        scope,
        name,
        result.as_ref().map(|p| p.name.clone())
    );
    result
}

/// Rewrite `dist.tarball` theo Host header của request hiện tại — store lưu URL
/// của registry cũ (host:port lúc publish); đọc từ host/port khác phải trả URL
/// đúng chỗ này, nếu không client fetch tarball ra registry sai.
/// Scheme follows `X-Forwarded-Proto` (TLS-terminating proxies like the
/// bundled nginx set it); without it we keep plain `http` (local/default
/// deployments serve HTTP directly — advertising `https` there would hand
/// clients an unreachable URL). Only `http`/`https` values are honored.
/// Dùng scheme từ proxy TLS; mặc định HTTP cho triển khai local và chỉ nhận HTTP/HTTPS.
pub(in crate::npm) fn rewrite_tarball_host(pkg: &mut Package, host: &str, scheme: &str) {
    let scheme = match scheme {
        "https" | "http" => scheme,
        _ => "http",
    };
    for v in pkg.versions.values_mut() {
        if let Some(filename) = v.dist.tarball.rsplit('/').next() {
            v.dist.tarball = format!("{scheme}://{host}/{}/-/{filename}", pkg.name);
        }
    }
}

pub(in crate::npm) async fn get_package(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    Path(name): Path<String>,
) -> Result<Json<Package>, StatusCode> {
    auth.authorize_package_read_async(&headers, &name).await?;
    match store.get_package(&name).await {
        Ok(Some(mut pkg)) => {
            let host = headers
                .get("host")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("localhost");
            let scheme = headers
                .get("x-forwarded-proto")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("http");
            rewrite_tarball_host(&mut pkg, host, scheme);
            Ok(Json(pkg))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            warn!("Failed to get package {}: {}", name, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

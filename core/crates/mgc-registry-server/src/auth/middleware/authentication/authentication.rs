//! Required authentication middleware.
// Giữ route public hiện có, từ chối malformed header và gắn user xác thực.

use crate::auth::{AuthService, unauthorized_response};
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use std::sync::Arc;

/// Auth middleware — dùng qua route_layer + Extension (không có State trong route_layer).
/// Trả 401 kèm WWW-Authenticate challenge (chuẩn OCI/Docker): client Basic/Bearer
/// đọc challenge rồi gửi credential — thiếu header khiến oras/pip từ chối retry.
pub async fn auth_middleware(
    axum::extract::Extension(auth): axum::extract::Extension<Arc<AuthService>>,
    request: Request,
    next: Next,
) -> Response {
    use axum::response::IntoResponse;
    // Adduser public (chuẩn npm): tạo credential mới, không cần token sẵn có
    if request.method() == axum::http::Method::PUT && request.uri().path().starts_with("/-/user/") {
        return next.run(request).await;
    }
    if request.method() == axum::http::Method::POST
        && request.uri().path() == "/-/v1/trusted-publish/token"
    {
        return next.run(request).await;
    }
    // Liveness probe, npm convention. Every "is the registry running?" error
    // message depends on this answering a caller without credentials; the body is
    // `{}`, so nothing is disclosed. Without the exemption the probe returned 401
    // whenever `--admin-token` was configured — the case where an operator most
    // needs it.
    // (Probe sống theo chuẩn npm. Mọi thông điệp "registry có chạy không?" cần nó
    // trả lời được mà không cần credential; body là `{}` nên không lộ gì. Thiếu
    // miễn trừ thì probe trả 401 khi có `--admin-token`.)
    if request.method() == axum::http::Method::GET && request.uri().path() == "/-/ping" {
        return next.run(request).await;
    }

    if request
        .headers()
        .get("authorization")
        .is_some_and(|value| value.to_str().is_err())
    {
        return StatusCode::BAD_REQUEST.into_response();
    }
    if let Some(user) = auth.authenticate_headers_async(request.headers()).await {
        let mut request = request;
        request.extensions_mut().insert(user);
        return next.run(request).await;
    }

    unauthorized_response()
}

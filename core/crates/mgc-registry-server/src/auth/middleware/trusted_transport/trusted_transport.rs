//! Trusted token transport middleware.
// Chặn token trusted trên transport chưa xác thực.

use crate::auth::AuthService;
use axum::{
    extract::{ConnectInfo, Extension, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::net::SocketAddr;
use std::sync::Arc;

/// Reject any request carrying an ephemeral trusted token over an unverified transport.
/// Từ chối mọi request mang token trusted tạm thời qua transport chưa được xác thực.
pub async fn trusted_transport_middleware(
    Extension(auth): Extension<Arc<AuthService>>,
    request: Request,
    next: Next,
) -> Response {
    let peer = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(address)| *address);
    if auth.has_active_trusted_token(request.headers())
        && !auth.trusted_transport_allowed(request.headers(), peer)
    {
        return StatusCode::FORBIDDEN.into_response();
    }
    next.run(request).await
}

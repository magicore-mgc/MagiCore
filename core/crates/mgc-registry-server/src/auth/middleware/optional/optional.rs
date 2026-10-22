//! Optional authentication middleware.
// Gắn user khi Bearer hợp lệ, còn lại tiếp tục request.

use crate::auth::AuthService;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

/// Optional auth - extract user if present
pub async fn optional_auth(
    State(auth): State<Arc<AuthService>>,
    request: Request,
    next: Next,
) -> Response {
    let auth_header = request.headers().get("authorization");

    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(auth_value) = auth_header
        && let Ok(auth_str) = auth_value.to_str()
        && let Some(token) = auth_str.strip_prefix("Bearer ")
        && let Some(user) = auth.verify_token(token)
    {
        let mut request = request;
        request.extensions_mut().insert(user);
        return next.run(request).await;
    }

    next.run(request).await
}

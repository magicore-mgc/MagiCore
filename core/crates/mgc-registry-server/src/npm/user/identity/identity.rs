//! Report the identity attached to a bearer token.
//! Trả về danh tính gắn với bearer token.

use crate::AppState;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Json,
};

pub(in crate::npm) async fn whoami(
    State((_, auth)): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(String::from);

    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(token) = token
        && let Some(user) = auth.verify_token(&token)
    {
        return Ok(Json(serde_json::json!({
            "username": user.name,
            "is_admin": user.is_admin
        })));
    }

    Err(StatusCode::UNAUTHORIZED)
}

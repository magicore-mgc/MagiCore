//! Revoke an npm token through the admin-only endpoint.
//! Thu hồi token npm qua endpoint chỉ dành cho admin.

use crate::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};

/// ITEM 6: revoke token — chỉ admin (admin_token)
pub(in crate::npm) async fn revoke_token(
    State((_, auth)): State<AppState>,
    Path(token): Path<String>,
    headers: HeaderMap,
) -> Result<StatusCode, StatusCode> {
    auth.authorize_admin_async(&headers).await?;
    if !auth.remove_token(&token).await.unwrap_or(false) {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

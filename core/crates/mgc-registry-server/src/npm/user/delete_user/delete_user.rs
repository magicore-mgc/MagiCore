//! Remove a user through the admin-only endpoint.
//! Xóa user qua endpoint chỉ dành cho admin.

use crate::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};

pub(in crate::npm) async fn delete_user(
    State((_, auth)): State<AppState>,
    Path(name): Path<String>,
    headers: HeaderMap,
) -> Result<StatusCode, StatusCode> {
    auth.authorize_admin_async(&headers).await?;
    let name = name
        .strip_prefix("org.couchdb.user:")
        .unwrap_or(&name)
        .to_string();
    if !auth.remove_user(&name).await.unwrap_or(false) {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

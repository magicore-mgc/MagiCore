//! Register an npm user under the server privilege policy.
//! Đăng ký user npm theo chính sách quyền của server.

use crate::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub(in crate::npm) struct AddUserBody {
    password: String,
    email: Option<String>,
    #[serde(default)]
    scopes: Vec<String>,
    #[serde(default)]
    role: Option<String>,
}

pub(in crate::npm) async fn adduser(
    State((_, auth)): State<AppState>,
    Path(name): Path<String>,
    headers: HeaderMap,
    Json(body): Json<AddUserBody>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Only administrators grant publishing rights; public accounts are viewers.
    // Chỉ admin cấp quyền publish; tài khoản công khai chỉ có quyền đọc.
    let privileged = auth
        .authenticate_headers_async(&headers)
        .await
        .is_some_and(|user| user.is_admin);
    if !privileged
        && (!body.scopes.is_empty() || body.role.as_deref().is_some_and(|r| r != "viewer"))
    {
        return Err(StatusCode::FORBIDDEN);
    }
    let name = name.strip_prefix("org.couchdb.user:").unwrap_or(&name);
    // Reject unknown roles instead of silently downgrading a provisioning request.
    // Báo lỗi role không hợp lệ để admin không tưởng đã cấp đúng quyền.
    let role = match body.role.as_deref().unwrap_or("viewer") {
        "viewer" => crate::auth::UserRole::Viewer,
        "publisher" => crate::auth::UserRole::Publisher,
        "admin" => crate::auth::UserRole::Admin,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    if name.trim().is_empty() || body.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let user = crate::auth::User {
        name: name.to_string(),
        is_admin: privileged && role == crate::auth::UserRole::Admin,
        role,
        scopes: body.scopes,
        password: Some(body.password),
        email: body.email,
    };
    let token = uuid::Uuid::new_v4().to_string();
    auth.register_user(token.clone(), user).await?;

    Ok(Json(serde_json::json!({
        "ok": true,
        "username": name,
        "token": token
    })))
}

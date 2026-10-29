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
    // SEC-16: a prefixed, full-entropy token. A bare UUID has no domain tag, so
    // a leaked token is indistinguishable from any other identifier in a log
    // and cannot be detected by secret scanners that key on known prefixes.
    // (SEC-16: token có prefix và đầy đủ entropy. UUID trần không có domain
    // tag, nên token rò rỉ không phân biệt được với id bất kỳ trong log và
    // không bị secret scanner nhận ra theo prefix đã biết.)
    let token = registration_token();
    auth.register_user(token.clone(), user).await?;

    Ok(Json(serde_json::json!({
        "ok": true,
        "username": name,
        "token": token
    })))
}

/// Registration token with the `mgc_` prefix and 256 bits of entropy.
/// Token đăng ký có prefix `mgc_` và 256 bit entropy.
///
/// `Uuid::new_v4` only carries 122 random bits; two v4 UUIDs are drawn from
/// the same pool as any other id, so prefixing makes the value self-identifying
/// in logs, proxies and secret scanners.
/// (`Uuid::new_v4` chỉ có 122 bit ngẫu nhiên; hai UUID v4 lấy từ cùng nguồn như
/// mọi id khác, nên prefix giúp giá trị tự nhận diện trong log, proxy và
/// scanner.)
pub(super) fn registration_token() -> String {
    // Two independent v4 UUIDs give 244 random bits behind a prefix that makes
    // the value recognisable in logs; `mgc-crypto` does not expose a raw
    // entropy source and a new dependency for one token is not justified.
    // (Hai UUID v4 độc lập cho 244 bit ngẫu nhiên sau prefix giúp giá trị nhận
    // ra được trong log; `mgc-crypto` không mở nguồn entropy thô và thêm
    // dependency cho một token là không tương xứng.)
    let token = format!(
        "mgc_{}{}",
        uuid::Uuid::new_v4().simple(),
        uuid::Uuid::new_v4().simple()
    );
    debug_assert_eq!(token.len(), 4 + 64);
    token
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

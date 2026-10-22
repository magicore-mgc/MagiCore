//! Registry user creation, deletion, and access-token revocation.
//! Tạo user, xóa user và thu hồi access token registry.

use mgc_config::npmrc::NpmRc;

use super::super::registry_url::{admin_client, revoke_token_url, user_url};
use super::error::{UserManagementError, UserManagementResult};

pub(in crate::commands::registry) async fn user_add(
    name: &str,
    password_stdin: bool,
    scopes: &[String],
    role: &str,
    registry: &str,
) -> UserManagementResult<()> {
    let password = crate::commands::login::read_password(password_stdin, "Password")
        .map_err(UserManagementError::PasswordInput)?;
    let token = crate::commands::login::create_registry_user(
        registry,
        name,
        &password,
        &serde_json::json!({ "email": "", "scopes": scopes, "role": role }),
    )
    .await
    .map_err(UserManagementError::UserCreation)?;
    let npmrc_path = dirs::home_dir()
        .ok_or_else(crate::error::no_home_dir)
        .map_err(UserManagementError::ConfigHome)?
        .join(".npmrc");
    NpmRc::save_auth_token_for_registry_url(&npmrc_path, registry, &token)
        .map_err(UserManagementError::TokenPersistence)?;
    println!(
        "User created; registry token saved to {}",
        npmrc_path.display()
    );
    Ok(())
}

pub(in crate::commands::registry) async fn user_rm(
    name: &str,
    registry: &str,
    admin_token: Option<String>,
) -> UserManagementResult<()> {
    let token = required_admin_token(admin_token)?;
    let url = user_url(registry, name)?;
    let client = admin_client(&url)?;
    let response = client
        .delete(url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(sanitize_request_error)?;
    let status = response.status();
    if status.as_u16() == 404 {
        return Err(UserManagementError::UserNotFound(name.to_owned()));
    }
    if !status.is_success() {
        return Err(UserManagementError::DeleteFailed(status.as_u16()));
    }
    println!("User {name} deleted");
    Ok(())
}

pub(in crate::commands::registry) async fn revoke_token(
    token: &str,
    registry: &str,
    admin_token: Option<String>,
) -> UserManagementResult<()> {
    let admin_token = required_admin_token(admin_token)?;
    if token.is_empty() {
        return Err(UserManagementError::EmptyToken);
    }
    let url = revoke_token_url(registry, token)?;
    let client = admin_client(&url)?;
    let response = client
        .delete(url)
        .bearer_auth(admin_token)
        .send()
        .await
        .map_err(sanitize_request_error)?;
    let status = response.status();
    if status.as_u16() == 404 {
        return Err(UserManagementError::TokenNotFound);
    }
    if !status.is_success() {
        return Err(UserManagementError::RevokeFailed(status.as_u16()));
    }
    println!("Token revoked");
    Ok(())
}

fn required_admin_token(token: Option<String>) -> UserManagementResult<String> {
    let Some(token) = token else {
        return Err(UserManagementError::AdminTokenRequired);
    };
    if token.is_empty() || !token.bytes().all(|byte| byte.is_ascii_graphic()) {
        return Err(UserManagementError::InvalidAdminToken);
    }
    Ok(token)
}

fn sanitize_request_error(error: reqwest::Error) -> UserManagementError {
    // The token identifier is part of the revocation URL; strip URL data before retaining cause.
    // Token nằm trong URL thu hồi; bỏ URL trước khi giữ lại nguyên nhân lỗi mạng.
    UserManagementError::RequestFailed(error.without_url())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

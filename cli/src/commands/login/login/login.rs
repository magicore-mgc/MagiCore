//! Coordinate registry selection, credential input, registration and token storage.
//! Điều phối chọn registry, nhập credential, đăng ký và lưu token.

use anyhow::Result;

use super::credentials::{prompt_text, read_password_inner};
use super::error::LoginError;
use super::registry_user_creation::create_registry_user_inner;
use super::token_storage::store_registry_token;
use super::types::LoginArgs;

const DEFAULT_REGISTRY: &str = "https://registry.npmjs.org/";

pub async fn run(args: LoginArgs) -> Result<()> {
    run_inner(args).await.map_err(LoginError::into_diagnostic)
}

async fn run_inner(args: LoginArgs) -> Result<(), LoginError> {
    // SEC-03: never take the credential destination from a project file.
    // A cloned repository can plant `.npmrc` with `registry=https://evil`, and
    // the password would be PUT to that host. Only explicit flag, user config
    // and environment decide where a password goes.
    // SEC-03: không lấy đích gửi credential từ file trong project.
    // Repo clone có thể đặt `.npmrc` trỏ host lạ và mật khẩu sẽ được PUT tới
    // đó. Chỉ flag tường minh, config của user và env quyết định đích.
    let registry = match args.registry {
        Some(url) => url,
        None => registry_or_default(mgc_config::npmrc::user_registry_only()),
    };

    let username = match args.username {
        Some(username) => username,
        None => prompt_text("Username")?,
    };
    if username.is_empty() {
        return Err(LoginError::UsernameRequired);
    }

    let password = read_password_inner(args.password_stdin, "Password")?;
    let token = create_registry_user_inner(
        &registry,
        &username,
        &password,
        &serde_json::json!({ "email": "" }),
    )
    .await?;

    let npmrc_path = store_registry_token(args.local, &registry, &token)?;
    println!("Login succeeded. Token saved to {}", npmrc_path.display());
    Ok(())
}

fn registry_or_default(configured: Option<String>) -> String {
    configured.unwrap_or_else(|| DEFAULT_REGISTRY.to_string())
}

#[cfg(test)]
#[path = "test/login.rs"]
mod tests;

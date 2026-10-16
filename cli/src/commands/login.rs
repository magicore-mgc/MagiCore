//! Login command — adduser flow, save token to ~/.npmrc (01 §1 line 34)
//! (Lệnh login: đăng ký/adduser vào registry, lưu token vào .npmrc)

use anyhow::Result;
use clap::Args;
use std::io::{BufRead, Read, Write};
use url::Url;

use mgc_config::npmrc::NpmRc;

/// Default registry (hardcode warning — OK: default const, overrideable via flag)
const DEFAULT_REGISTRY: &str = "https://registry.npmjs.org/";
const REGISTRY_AUTH_TIMEOUT_SECS: u64 = 15;
const MAX_REGISTRY_AUTH_RESPONSE_BYTES: usize = 32 * 1024;
const MAX_PASSWORD_STDIN_BYTES: usize = 8 * 1024;

#[derive(Args, Debug, Clone)]
pub struct LoginArgs {
    #[arg(long, help = "registry URL (default: config or npmjs)")]
    pub registry: Option<String>,
    #[arg(long, help = "username (prompt if omitted)")]
    pub username: Option<String>,
    #[arg(long, help = "read password from stdin; otherwise prompt without echo")]
    pub password_stdin: bool,
    #[arg(long, help = "write token to project .npmrc instead of ~/.npmrc")]
    pub local: bool,
}

/// Prompt for a visible, non-secret value such as a registry username.
/// Nhập trường không bí mật như username và cho phép hiển thị ký tự.
fn prompt(label: &str) -> Result<String> {
    print!("{}: ", label);
    std::io::stdout().flush()?;
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

pub async fn run(args: LoginArgs) -> Result<()> {
    let registry = match args.registry {
        Some(url) => url,
        None => NpmRc::load(&std::env::current_dir()?)
            .ok()
            .and_then(|rc| rc.registry)
            .unwrap_or_else(|| DEFAULT_REGISTRY.to_string()),
    };

    let username = match args.username {
        Some(u) => u,
        None => prompt("Username")?,
    };
    if username.is_empty() {
        return Err(crate::error::registry_auth_username_required());
    }
    let password = read_password(args.password_stdin, "Password")?;
    let token = create_registry_user(
        &registry,
        &username,
        &password,
        &serde_json::json!({ "email": "" }),
    )
    .await?;

    let npmrc_path = if args.local {
        std::env::current_dir()?.join(".npmrc")
    } else {
        dirs::home_dir()
            .ok_or_else(crate::error::no_home_dir)?
            .join(".npmrc")
    };

    NpmRc::save_auth_token_for_registry_url(&npmrc_path, &registry, &token)?;
    println!("Login succeeded. Token saved to {}", npmrc_path.display());

    Ok(())
}

/// Obtain a secret from a hidden terminal prompt or an explicitly piped stdin.
/// Lấy bí mật qua prompt ẩn hoặc stdin khi caller chọn cờ tường minh.
pub(crate) fn read_password(password_stdin: bool, label: &str) -> Result<String> {
    let password = if password_stdin {
        let mut value = String::new();
        std::io::stdin()
            .take((MAX_PASSWORD_STDIN_BYTES + 1) as u64)
            .read_to_string(&mut value)?;
        if value.len() > MAX_PASSWORD_STDIN_BYTES {
            return Err(crate::error::registry_auth_password_too_large());
        }
        value.trim_end_matches(['\r', '\n']).to_owned()
    } else {
        mgc_ui::prompt::password(label)?
    };
    if password.is_empty() {
        return Err(crate::error::registry_auth_password_required());
    }
    Ok(password)
}

/// Build an encoded user endpoint while preserving a registry path prefix.
/// Tạo endpoint đã encode username và giữ nguyên path prefix của registry.
fn registry_user_url(registry: &str, username: &str) -> Result<Url> {
    let mut url = Url::parse(registry).map_err(|_| crate::error::registry_auth_url_invalid())?;
    let host = url
        .host_str()
        .ok_or_else(crate::error::registry_auth_url_invalid)?;
    if !matches!(url.scheme(), "https" | "http")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(crate::error::registry_auth_url_invalid());
    }
    if url.scheme() != "https" && !is_loopback_host(host) {
        return Err(crate::error::registry_auth_requires_https());
    }
    let user_segment = format!("org.couchdb.user:{username}");
    url.path_segments_mut()
        .map_err(|_| crate::error::registry_auth_url_invalid())?
        .pop_if_empty()
        .extend(["-", "user", user_segment.as_str()]);
    Ok(url)
}

fn is_loopback_host(host: &str) -> bool {
    host.eq_ignore_ascii_case("localhost")
        || host
            .trim_start_matches('[')
            .trim_end_matches(']')
            .parse::<std::net::IpAddr>()
            .is_ok_and(|address| address.is_loopback())
}

/// Register a user without exposing response bodies or following credential-bearing redirects.
/// Tạo user không làm lộ response body và không chuyển credential qua redirect.
pub(crate) async fn create_registry_user(
    registry: &str,
    username: &str,
    password: &str,
    extra_fields: &serde_json::Value,
) -> Result<String> {
    let url = registry_user_url(registry, username)?;
    let mut body = serde_json::json!({
        "name": username,
        "password": password,
    });
    if let (Some(body), Some(extra)) = (body.as_object_mut(), extra_fields.as_object()) {
        body.extend(
            extra
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        );
    }
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(REGISTRY_AUTH_TIMEOUT_SECS))
        .build()?;
    let response = client
        .put(url)
        .json(&body)
        .send()
        .await
        .map_err(|_| crate::error::registry_auth_request_failed())?;
    let status = response.status();
    if !status.is_success() {
        return Err(crate::error::registry_auth_status_failed(status.as_u16()));
    }
    if response
        .content_length()
        .is_some_and(|size| size > MAX_REGISTRY_AUTH_RESPONSE_BYTES as u64)
    {
        return Err(crate::error::registry_auth_response_too_large());
    }
    let mut response = response;
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| crate::error::registry_auth_request_failed())?
    {
        if bytes.len().saturating_add(chunk.len()) > MAX_REGISTRY_AUTH_RESPONSE_BYTES {
            return Err(crate::error::registry_auth_response_too_large());
        }
        bytes.extend_from_slice(&chunk);
    }
    let parsed: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|_| crate::error::registry_auth_response_invalid())?;
    let token = parsed["token"]
        .as_str()
        .filter(|token| {
            !token.is_empty()
                && token.len() <= MAX_REGISTRY_AUTH_RESPONSE_BYTES
                && token
                    .bytes()
                    .all(|byte| !byte.is_ascii_whitespace() && !byte.is_ascii_control())
        })
        .ok_or_else(crate::error::no_token_in_response)?;
    Ok(token.to_owned())
}

#[cfg(test)]
#[path = "test/login.rs"]
mod tests;

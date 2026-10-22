//! Registry authentication and metadata diagnostics.
//! Lỗi xác thực registry và metadata.

use anyhow::Error;
use anyhow::anyhow;

pub fn registry_missing_field(field: &str) -> Error {
    anyhow!("Registry response missing {field}")
}

pub fn no_token_in_response() -> Error {
    anyhow!("registry response did not contain a usable token")
}

pub fn registry_auth_url_invalid() -> Error {
    anyhow!(
        "registry authentication requires a valid HTTP(S) URL without credentials, query, or fragment"
    )
}

pub fn registry_auth_requires_https() -> Error {
    anyhow!("registry authentication requires HTTPS except for loopback registry URLs")
}

pub fn registry_auth_request_failed() -> Error {
    anyhow!("registry authentication request failed or exceeded its timeout")
}

pub fn registry_auth_status_failed(status: u16) -> Error {
    anyhow!("registry authentication failed with HTTP {status}")
}

pub fn registry_auth_response_too_large() -> Error {
    anyhow!("registry authentication response exceeds the size limit")
}

pub fn registry_auth_response_invalid() -> Error {
    anyhow!("registry authentication response is not valid JSON")
}

pub fn registry_auth_password_required() -> Error {
    anyhow!("password input was empty")
}

pub fn registry_auth_username_required() -> Error {
    anyhow!("username is required")
}

pub fn registry_auth_password_too_large() -> Error {
    anyhow!("password input exceeds the size limit")
}

pub fn no_home_dir() -> Error {
    anyhow!("No home directory")
}

pub fn registry_fetch_failed(package: &str, e: &dyn std::fmt::Display) -> Error {
    let _ = package;
    anyhow!("Could not fetch package info from registry: {e}")
}

pub fn registry_url_https() -> Error {
    anyhow!("registry URL must use HTTPS unless it targets localhost")
}

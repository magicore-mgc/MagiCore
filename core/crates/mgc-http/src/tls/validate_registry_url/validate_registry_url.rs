//! Enforces the existing registry URL acceptance policy.
//! Giữ nguyên policy chấp nhận URL registry hiện tại.

use anyhow::{Result, bail};

/// Validate URL scheme; HTTPS is required for remote registries.
/// Kiểm tra URL; registry từ xa phải dùng HTTPS.
pub fn validate_registry_url(url: &str, allow_http_local: bool) -> Result<()> {
    let parsed = url::Url::parse(url)?;
    if parsed.scheme() == "http" {
        if !allow_http_local {
            bail!("HTTP not allowed for remote registry — use HTTPS");
        }
        let host = parsed.host_str().unwrap_or("");
        if host != "localhost" && host != "127.0.0.1" {
            bail!("HTTP only allowed for localhost/127.0.0.1 — use HTTPS for remote");
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/validate_registry_url.rs"]
mod tests;

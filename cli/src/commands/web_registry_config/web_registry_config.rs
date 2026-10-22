//! Validate Web registry configuration and build npm-compatible endpoints.
//! Xác thực cấu hình Web registry và tạo endpoint tương thích npm.

use anyhow::Result;
use std::ffi::OsString;
use url::Url;

use super::error::WebRegistryConfigError;
use crate::commands::registry::registry_url::{RegistryUrlError, validated_registry_base};

pub const DEFAULT_WEB_REGISTRY_URL: &str = "https://registry.npmjs.org";
pub const WEB_REGISTRY_URL_ENV: &str = "MAGICORE_WEB_REGISTRY_URL";

/// Return the configured registry only after validating its transport and URL components.
/// Chỉ trả registry đã cấu hình sau khi xác thực transport và thành phần URL.
pub fn web_registry_url() -> Result<String> {
    let registry = configured_web_registry_url(std::env::var_os(WEB_REGISTRY_URL_ENV))?;
    validated_web_registry_url(&registry)?;
    Ok(registry)
}

fn configured_web_registry_url(value: Option<OsString>) -> Result<String, WebRegistryConfigError> {
    match value {
        Some(value) => value
            .into_string()
            .map_err(|_| WebRegistryConfigError::InvalidEnvironmentValue),
        None => Ok(DEFAULT_WEB_REGISTRY_URL.to_string()),
    }
}

pub fn search_endpoint(base: &str, query: &str, size: u32, from: u32) -> Result<String> {
    let mut url = join_registry_path(base, "-/v1/search")?;
    url.query_pairs_mut()
        .append_pair("text", query)
        .append_pair("size", &size.to_string())
        .append_pair("from", &from.to_string());
    Ok(url.to_string())
}

pub fn advisory_bulk_endpoint(base: &str) -> Result<String> {
    Ok(join_registry_path(base, "-/npm/v1/security/advisories/bulk")?.to_string())
}

/// Validate a registry URL with the shared strict scheme/authority policy.
/// Xác thực URL registry bằng policy scheme/authority nghiêm ngặt dùng chung.
fn validated_web_registry_url(base: &str) -> Result<Url, WebRegistryConfigError> {
    validated_registry_base(base).map_err(map_registry_url_error)
}

pub fn join_registry_path(base: &str, path: &str) -> Result<Url> {
    let mut url = validated_web_registry_url(base)?;
    let base_path = url.path().trim_end_matches('/');
    let path = path.trim_start_matches('/');
    let joined = if base_path.is_empty() || base_path == "/" {
        format!("/{path}")
    } else {
        format!("{base_path}/{path}")
    };
    url.set_path(&joined);
    Ok(url)
}

fn map_registry_url_error(error: RegistryUrlError) -> WebRegistryConfigError {
    match error {
        RegistryUrlError::RequiresHttps => WebRegistryConfigError::RequiresHttps,
        error => WebRegistryConfigError::InvalidRegistry(error),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

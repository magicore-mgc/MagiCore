//! Validate registry bases and construct safe registry endpoints and clients.
//! Xác thực URL gốc và tạo endpoint/client registry an toàn.

use std::time::Duration;

use reqwest::redirect::Policy;
use url::Url;

use super::error::RegistryUrlError;

const DEFAULT_ADMIN_TIMEOUT_SECS: u64 = 15;
const MAX_ADMIN_TIMEOUT_SECS: u64 = 300;
const ADMIN_TIMEOUT_ENV: &str = "MGC_REGISTRY_ADMIN_TIMEOUT_SECS";

/// Parse a registry base after rejecting user-info and URL request components.
/// Đọc URL gốc sau khi chặn user-info và thành phần request gắn kèm.
pub(crate) fn validated_registry_base(registry: &str) -> Result<Url, RegistryUrlError> {
    let mut url = Url::parse(registry).map_err(|_| RegistryUrlError::InvalidRegistry)?;
    if !matches!(url.scheme(), "https" | "http")
        || raw_authority_has_user_info(registry)
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.host_str().is_none()
    {
        return Err(RegistryUrlError::InvalidRegistry);
    }
    if url.scheme() != "https" && !is_loopback_host(url.host_str().unwrap_or_default()) {
        return Err(RegistryUrlError::RequiresHttps);
    }
    url.set_query(None);
    url.set_fragment(None);
    Ok(url)
}

fn raw_authority_has_user_info(registry: &str) -> bool {
    let Some((_, after_scheme)) = registry.split_once("://") else {
        return false;
    };
    let authority = after_scheme
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default();
    authority.contains('@')
}

fn is_loopback_host(host: &str) -> bool {
    host.eq_ignore_ascii_case("localhost")
        || host
            .trim_start_matches('[')
            .trim_end_matches(']')
            .parse::<std::net::IpAddr>()
            .is_ok_and(|address| address.is_loopback())
}

fn append_segments(mut base: Url, segments: &[&str]) -> Result<Url, RegistryUrlError> {
    let mut path = base
        .path_segments_mut()
        .map_err(|_| RegistryUrlError::InvalidPath)?;
    path.pop_if_empty();
    path.extend(segments.iter().copied());
    drop(path);
    Ok(base)
}

/// Build the CouchDB-compatible user endpoint while preserving registry prefix.
/// Tạo endpoint user tương thích CouchDB và giữ nguyên prefix registry.
pub(crate) fn user_url(registry: &str, username: &str) -> Result<Url, RegistryUrlError> {
    let base = validated_registry_base(registry)?;
    let user_segment = format!("org.couchdb.user:{username}");
    append_segments(base, &["-", "user", user_segment.as_str()])
}

/// Build the trusted publisher binding endpoint.
/// Tạo endpoint trusted publisher binding.
pub(crate) fn binding_url(registry: &str) -> Result<Url, RegistryUrlError> {
    append_segments(
        validated_registry_base(registry)?,
        &["-", "v1", "trusted-publish", "bindings"],
    )
}

/// Build the token revocation endpoint with the opaque token in one encoded segment.
/// Tạo endpoint thu hồi token, giữ token opaque trong một path segment đã encode.
pub(crate) fn revoke_token_url(registry: &str, token: &str) -> Result<Url, RegistryUrlError> {
    append_segments(
        validated_registry_base(registry)?,
        &["-", "user", "token", token],
    )
}

/// Build a client that never forwards credentials through an HTTP redirect.
/// Tạo client tuyệt đối không chuyển tiếp credential qua HTTP redirect.
pub(crate) fn client_for_url(
    url: &Url,
    timeout: Duration,
) -> Result<reqwest::Client, RegistryUrlError> {
    let builder = reqwest::Client::builder()
        .redirect(Policy::none())
        .timeout(timeout);
    // Local admin traffic must not escape through ambient HTTP(S)_PROXY settings.
    // Request admin cục bộ không được đi qua HTTP(S)_PROXY do môi trường cấu hình.
    let builder = if should_bypass_system_proxy(url) {
        builder.no_proxy()
    } else {
        builder
    };
    builder.build().map_err(RegistryUrlError::ClientBuild)
}

fn should_bypass_system_proxy(url: &Url) -> bool {
    url.host_str().is_some_and(is_loopback_host)
}

fn admin_timeout() -> Result<Duration, RegistryUrlError> {
    let Some(value) = std::env::var_os(ADMIN_TIMEOUT_ENV) else {
        return Ok(Duration::from_secs(DEFAULT_ADMIN_TIMEOUT_SECS));
    };
    let seconds = value
        .to_str()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|seconds| (1..=MAX_ADMIN_TIMEOUT_SECS).contains(seconds))
        .ok_or(RegistryUrlError::InvalidAdminTimeout)?;
    Ok(Duration::from_secs(seconds))
}

/// Build the bounded admin client using the configured timeout.
/// Tạo client admin với timeout có giới hạn và đọc từ cấu hình.
pub(crate) fn admin_client(url: &Url) -> Result<reqwest::Client, RegistryUrlError> {
    client_for_url(url, admin_timeout()?)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

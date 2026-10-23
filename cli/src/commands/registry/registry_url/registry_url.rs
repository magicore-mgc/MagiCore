//! Validate registry bases and construct safe registry endpoints and clients.
//! Xác thực URL gốc và tạo endpoint/client registry an toàn.
use url::Url;

use super::error::RegistryUrlError;

#[path = "client/mod.rs"]
pub(crate) mod client;

pub(crate) use client::{admin_client, client_for_url};

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

pub(crate) fn is_loopback_host(host: &str) -> bool {
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
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

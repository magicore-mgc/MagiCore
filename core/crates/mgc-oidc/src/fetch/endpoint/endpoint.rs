//! Validate endpoint URLs and retrieve bounded JSON — kiểm tra URL và tải JSON có giới hạn.
//!
use crate::error::OidcError;
use crate::fetch::limits::{MAX_RESPONSE_BYTES, REQUEST_TIMEOUT_SECS};
use mgc_http::{HttpClient, TlsConfig, timeout::TimeoutConfig};
use std::time::Duration;
use url::Url;

/// HTTPS only, without URL credentials — chỉ HTTPS, không nhận credential trong URL.
pub(crate) fn secure_url(raw: &str) -> Result<Url, OidcError> {
    let url = Url::parse(raw)
        .map_err(|_| OidcError::FetchFailed("invalid identity endpoint URL".into()))?;
    if url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err(OidcError::FetchFailed(
            "identity endpoint requires HTTPS without credentials or fragment".into(),
        ));
    }
    Ok(url)
}

/// Bound time, redirects and response size; never expose endpoint URLs in errors.
/// Giới hạn thời gian, redirect, kích thước; không đưa URL nhạy cảm vào lỗi.
pub(crate) async fn fetch_json(
    url: Url,
    bearer: Option<&str>,
) -> Result<serde_json::Value, OidcError> {
    let timeout = TimeoutConfig {
        connect: Duration::from_secs(REQUEST_TIMEOUT_SECS),
        request: Duration::from_secs(REQUEST_TIMEOUT_SECS),
        ..TimeoutConfig::default()
    };
    let client = HttpClient::with_security_no_redirects(&timeout, &TlsConfig::default())
        .map_err(|_| OidcError::FetchFailed("cannot initialize identity client".into()))?;
    let client = if let Some(token) = bearer {
        client.with_auth("authorization", format!("Bearer {token}"))
    } else {
        client
    };
    let response = tokio::time::timeout(
        Duration::from_secs(REQUEST_TIMEOUT_SECS),
        client.get_bytes_limited(url.as_str(), MAX_RESPONSE_BYTES),
    )
    .await
    .map_err(|_| OidcError::FetchFailed("identity request timed out".into()))?
    .map_err(|_| OidcError::FetchFailed("identity request failed".into()))?;
    let (status, body) = response;
    if !(200..300).contains(&status) {
        return Err(OidcError::FetchFailed(format!(
            "identity endpoint returned {status}"
        )));
    }
    serde_json::from_slice(&body)
        .map_err(|_| OidcError::FetchFailed("invalid identity JSON response".into()))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

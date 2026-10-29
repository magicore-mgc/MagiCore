//! Shared HTTP client state, configuration, and convenience methods.
//! State, cấu hình và các method tiện ích của HTTP client dùng chung.

use super::error::HttpClientError;
use crate::{
    cache::HttpCache,
    ratelimit::RateLimiter,
    retry::RetryStrategy,
    timeout::{TimeoutConfig, apply_timeouts},
    tls::TlsConfig,
};
use anyhow::Result;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

/// Shared HTTP client state for transport and request policy.
/// State client HTTP dùng chung cho transport và policy request.
#[derive(Clone)]
pub struct HttpClient {
    pub(super) client: Client,
    pub(super) retry: RetryStrategy,
    pub(super) ratelimit: Option<RateLimiter>,
    pub(super) cache: Option<HttpCache>,
    pub(super) auth: Option<(String, String)>,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        Self::with_security(&TimeoutConfig::default(), &TlsConfig::default())
    }

    /// Build an HTTP client with explicit timeout and TLS policy.
    /// Tạo client với policy timeout và TLS tường minh.
    pub fn with_security(timeout: &TimeoutConfig, tls: &TlsConfig) -> Result<Self> {
        Self::with_builder(Client::builder(), timeout, tls)
    }

    /// Build a client that refuses redirects for identity and credential requests.
    /// Tạo client từ chối redirect cho request định danh và credential.
    pub fn with_security_no_redirects(timeout: &TimeoutConfig, tls: &TlsConfig) -> Result<Self> {
        let builder = Client::builder().redirect(super::redirect_policy::no_redirect_policy());
        Self::with_builder_keeping_redirect(builder, timeout, tls)
    }

    /// SEC-8/G5: the default redirect policy is same-origin with a hop cap so a
    /// static `Authorization` header is never forwarded to another host.
    /// (SEC-8/G5: policy redirect mặc định là cùng origin có giới hạn hop để
    /// header `Authorization` tĩnh không bao giờ bị chuyển sang host khác.)
    fn with_builder(
        builder: ClientBuilder,
        timeout: &TimeoutConfig,
        tls: &TlsConfig,
    ) -> Result<Self> {
        let builder = builder.redirect(super::redirect_policy::same_origin_policy(
            super::redirect_policy::DEFAULT_MAX_REDIRECTS,
        ));
        Self::with_builder_keeping_redirect(builder, timeout, tls)
    }

    /// Apply timeout/TLS to a builder that already carries its redirect policy.
    /// Áp timeout/TLS cho builder đã có policy redirect riêng.
    fn with_builder_keeping_redirect(
        builder: ClientBuilder,
        timeout: &TimeoutConfig,
        tls: &TlsConfig,
    ) -> Result<Self> {
        let builder = apply_timeouts(builder, timeout);
        let builder = tls.apply(builder)?;
        let client = builder
            .build()
            .map_err(|error| anyhow::Error::new(HttpClientError::from(error)))?;
        Ok(Self::from_client(client))
    }

    fn from_client(client: Client) -> Self {
        Self {
            client,
            retry: RetryStrategy::Exponential {
                base: Duration::from_secs(1),
                max: Duration::from_secs(30),
            },
            ratelimit: None,
            cache: None,
            auth: None,
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("HttpClient::new")
    }
}

/// Keep unstable inner transport details out of Debug output.
/// Không đưa chi tiết nội bộ transport thiếu ổn định vào Debug.
impl std::fmt::Debug for HttpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpClient").finish_non_exhaustive()
    }
}

#[cfg(test)]
#[path = "test/http_client.rs"]
mod tests;

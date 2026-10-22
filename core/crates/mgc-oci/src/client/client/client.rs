//! OCI client state, constructor, and authentication builder.
//! State OCI client, constructor và builder xác thực.

use anyhow::Result;
use mgc_http::{HttpClient, TlsConfig, timeout::TimeoutConfig};

/// OCI Client for registry operations
pub struct OciClient {
    pub(in crate::client) client: HttpClient,
    pub(in crate::client) base_url: String,
}

impl OciClient {
    pub fn new(registry_url: impl Into<String>, tls: Option<TlsConfig>) -> Result<Self> {
        let url = registry_url.into();
        let tls = tls.unwrap_or_default();
        let timeout = TimeoutConfig::default();
        let http = HttpClient::with_security(&timeout, &tls)?.with_retry(
            mgc_http::retry::RetryStrategy::Exponential {
                base: std::time::Duration::from_secs(1),
                max: std::time::Duration::from_secs(30),
            },
        );
        Ok(Self {
            client: http,
            base_url: url,
        })
    }

    /// Bearer token cho registry private (fail-closed khi server cấu hình admin token)
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.client = self
            .client
            .with_auth("authorization", format!("Bearer {}", token.into()));
        self
    }
}

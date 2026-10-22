//! Configures request authentication, retry, rate limiting, and cache state.
//! Cấu hình xác thực, retry, rate-limit và state cache của request.

use super::super::http_client::HttpClient;
use crate::{
    cache::HttpCache,
    ratelimit::{RateLimitConfig, RateLimiter},
    retry::RetryStrategy,
};
use std::time::Duration;

impl HttpClient {
    /// Attach a static auth header to every request.
    /// Gắn header xác thực tĩnh vào mọi request.
    pub fn with_auth(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.auth = Some((name.into(), value.into()));
        self
    }

    pub fn with_retry(mut self, strategy: RetryStrategy) -> Self {
        self.retry = strategy;
        self
    }

    pub fn with_ratelimit(mut self, max_req: u32, per: Duration) -> Self {
        self.ratelimit = Some(RateLimiter::new(RateLimitConfig {
            max_requests: max_req,
            period: per,
        }));
        self
    }

    pub fn with_cache(mut self, cache: HttpCache) -> Self {
        self.cache = Some(cache);
        self
    }
}

#[cfg(test)]
#[path = "test/request_configuration.rs"]
mod tests;

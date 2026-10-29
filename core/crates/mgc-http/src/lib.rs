pub mod cache;
pub mod methods;
pub mod offline;
pub mod proxy;
pub mod ratelimit;
pub mod retry;
pub mod telemetry;
pub mod timeout;
pub mod tls;
pub mod upload;

pub use cache::HttpCache;
pub use methods::{
    ALLOW_CROSS_ORIGIN_REDIRECTS_ENV, BoundedResponseError, DEFAULT_MAX_REDIRECTS, HttpClient,
    same_origin_policy,
};
pub use ratelimit::RateLimiter;
pub use tls::TlsConfig;

#[cfg(test)]
#[path = "test/lib_test.rs"]
mod tests;

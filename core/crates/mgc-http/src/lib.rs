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

/// Standard User-Agent header value for all MagiCore HTTP requests.
///
/// Format: `MagiCore/{version}` where version is the workspace release.
/// Used consistently across registry, search, advisory, and download clients.
/// (Giá trị User-Agent chuẩn cho mọi HTTP request MagiCore. Format thống nhất.)
pub const USER_AGENT: &str = concat!("MagiCore/", env!("CARGO_PKG_VERSION"));

/// User-Agent for batch/streaming operations (large tarballs, bulk downloads).
///
/// Format: `MagiCore/{version}/batch` - signals to servers that this is a
/// high-throughput streaming request that may need different rate limits.
/// (User-Agent cho batch/streaming — request throughput cao cần rate limit khác.)
pub const USER_AGENT_BATCH: &str = concat!("MagiCore/", env!("CARGO_PKG_VERSION"), "/batch");

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

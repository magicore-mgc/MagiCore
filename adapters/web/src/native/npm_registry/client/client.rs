//! HTTP client owners.
// Tạo và tái sử dụng HTTP client.

use crate::native::npm_registry::{client_builder_error, registry_redirect_policy};
use std::time::Duration;

/// Primary HTTP client: used for metadata fetches. Optimized for many concurrent
/// short-lived requests against the same host (registry.npmjs.org).
/// Built once and cheaply cloned (`Client` is reference-counted); a build
/// failure (broken TLS backend) is an explicit error, never a
/// process-wide panic.
pub(in crate::native::npm_registry) fn global_http_client() -> mgc_types::MgResult<reqwest::Client>
{
    use mgc_types::MgError;
    static CLIENT: std::sync::OnceLock<Result<reqwest::Client, String>> =
        std::sync::OnceLock::new();
    CLIENT
        .get_or_init(|| {
            reqwest::Client::builder()
                .redirect(registry_redirect_policy())
                // Increased pool: 256 concurrent connections per registry host.
                .pool_max_idle_per_host(256)
                .pool_idle_timeout(Duration::from_secs(90))
                .tcp_keepalive(Duration::from_secs(30))
                // Metadata responses are small; 30s is plenty.
                .timeout(Duration::from_secs(30))
                .user_agent(format!("MagiCore/{}", env!("CARGO_PKG_VERSION")))
                // H2 stream window: 4 MiB — allows multiple concurrent streams without
                // stalling when one response is slow.
                .http2_initial_stream_window_size(4 * 1024 * 1024)
                // H2 connection window: 32 MiB
                .http2_initial_connection_window_size(32 * 1024 * 1024)
                .build()
                .map_err(|error| client_builder_error("cannot build HTTP client", error))
        })
        .clone()
        .map_err(MgError::Other)
}

/// Batch HTTP client: used for tarball streaming. Optimized for large bodies.
/// Same fail-closed construction as [`global_http_client`].
pub fn batch_http_client() -> mgc_types::MgResult<reqwest::Client> {
    use mgc_types::MgError;
    static BATCH_CLIENT: std::sync::OnceLock<Result<reqwest::Client, String>> =
        std::sync::OnceLock::new();
    BATCH_CLIENT
        .get_or_init(|| {
            reqwest::Client::builder()
                .redirect(registry_redirect_policy())
                .pool_max_idle_per_host(256)
                .pool_idle_timeout(Duration::from_secs(300))
                .tcp_keepalive(Duration::from_secs(60))
                // Long timeout for massive tarballs like @next/swc (>200 MB).
                .timeout(Duration::from_secs(300))
                .user_agent(format!("MagiCore/{}/batch", env!("CARGO_PKG_VERSION")))
                .http2_initial_stream_window_size(16 * 1024 * 1024)
                .http2_initial_connection_window_size(64 * 1024 * 1024)
                .build()
                .map_err(|error| client_builder_error("cannot build batch HTTP client", error))
        })
        .clone()
        .map_err(MgError::Other)
}

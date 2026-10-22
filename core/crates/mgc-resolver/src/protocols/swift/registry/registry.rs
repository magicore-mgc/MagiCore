//! Swift registry state and HTTP helpers. / State registry Swift và helper HTTP.

use super::zip_cache::SwiftZipCache;
use mgc_types::{MgError, MgResult};

pub(super) const MAX_SWIFT_VERSION_LIST_BYTES: usize = 16 * 1024 * 1024;
pub(super) const MAX_SWIFT_CHECKSUM_BYTES: usize = 1024;
pub(super) const MAX_SWIFT_ARCHIVE_BYTES: usize = 512 * 1024 * 1024;

/// Native SwiftPM registry engine.
/// Engine native SwiftPM registry.
#[derive(Debug)]
pub struct SwiftRegistryProtocol {
    pub(super) registry_base: String,
    client: mgc_http::HttpClient,
    // Retains only bounded resolved archives for the same resolve/install flow.
    // (Chỉ giữ archive đã resolve trong giới hạn để dùng lại cùng lifecycle.)
    pub(super) zip_cache: SwiftZipCache,
}

impl SwiftRegistryProtocol {
    /// Build with an explicit registry base (testability). The base is the
    /// registry service root (no trailing slash).
    /// Dựng với registry base tường minh (cho test). Base là gốc dịch vụ
    /// registry (không dấu sẹo cuối).
    pub fn with_registry(registry_base: &str) -> Self {
        Self {
            registry_base: registry_base.trim_end_matches('/').to_string(),
            client: mgc_http::HttpClient::default(),
            zip_cache: SwiftZipCache::default(),
        }
    }

    /// Build from environment: `MGC_SWIFT_REGISTRY_URL`. SwiftPM has no
    /// universal default registry — an unset variable yields a protocol
    /// whose every resolve fails closed with explicit guidance (never a
    /// silent empty graph).
    /// Dựng từ môi trường: `MGC_SWIFT_REGISTRY_URL`. SwiftPM không có
    /// registry mặc định toàn cục — biến không đặt cho ra protocol mà mọi
    /// resolve fail-closed kèm hướng dẫn rõ ràng (không bao giờ graph rỗng
    /// âm thầm).
    pub fn from_env() -> Self {
        let zip_cache = SwiftZipCache::from_env();
        match std::env::var("MGC_SWIFT_REGISTRY_URL")
            .ok()
            .filter(|v| !v.trim().is_empty())
        {
            Some(url) => Self::with_registry_and_cache(url.trim(), zip_cache),
            None => {
                eprintln!(
                    "WARNING: MGC_SWIFT_REGISTRY_URL is not set — Swift registry \
                     resolve calls will fail closed (SwiftPM has no default registry)"
                );
                Self::with_registry_and_cache("", zip_cache)
            }
        }
    }

    fn with_registry_and_cache(registry_base: &str, zip_cache: SwiftZipCache) -> Self {
        Self {
            registry_base: registry_base.trim_end_matches('/').to_string(),
            client: mgc_http::HttpClient::default(),
            zip_cache,
        }
    }

    /// Host[:port] of the configured registry base (lock provenance tag).
    /// Host[:port] của registry base đã cấu hình (tag provenance lock).
    pub fn registry_host(&self) -> Option<String> {
        let base = self.registry_base.as_str();
        let rest = base.split_once("://").map(|(_, r)| r).unwrap_or(base);
        let host = rest
            .split('/')
            .next()
            .unwrap_or_default()
            .trim_end_matches('/');
        if host.is_empty() {
            None
        } else {
            Some(host.to_string())
        }
    }

    pub(super) async fn get_bytes(&self, url: &str, max_bytes: usize) -> MgResult<(u16, Vec<u8>)> {
        self.client
            .get_bytes_limited(url, max_bytes)
            .await
            .map_err(|error| MgError::Network(format!("GET {url} failed: {error}")))
    }

    /// `{registry}/{scope}/{name}/{file}` URL for a registry artifact.
    /// URL `{registry}/{scope}/{name}/{file}` cho artifact registry.
    pub(in crate::protocols::swift) fn registry_url_for(
        &self,
        dep_name: &str,
        file: &str,
    ) -> String {
        format!("{}/{}", self.registry_base, dep_name) + "/" + file
    }

    pub(in crate::protocols::swift) fn registry_base(&self) -> &str {
        &self.registry_base
    }
}

#[cfg(test)]
#[path = "test/registry_url/mod.rs"]
mod test;

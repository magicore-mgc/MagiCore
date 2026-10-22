//! Stable Crates.io RegistryProtocol facade.
//! Facade RegistryProtocol Crates.io ổn định.

use super::{
    DEFAULT_DOWNLOAD_BASE, DEFAULT_INDEX_URL, RegistryProtocol, RegistryRoot, ResolvedEntry,
};
use async_trait::async_trait;
use mgc_types::{MgResult, Version};
use std::path::PathBuf;
use std::time::Duration;

/// Native crates.io sparse-index engine.
/// Engine native sparse-index crates.io.
#[derive(Debug, Clone)]
pub struct CratesProtocol {
    pub(super) index_url: String,
    pub(super) download_base: String,
    pub(super) client: mgc_http::HttpClient,
}

impl CratesProtocol {
    /// Build with an explicit index URL (default download base).
    /// Dựng với index URL tường minh (download base mặc định).
    pub fn new(index_url: &str) -> Self {
        Self::with_download_base(index_url, DEFAULT_DOWNLOAD_BASE)
    }

    /// Build with explicit index + download base (testability).
    /// Dựng với index + download base tường minh (cho test).
    pub fn with_download_base(index_url: &str, download_base: &str) -> Self {
        let timeout = mgc_http::timeout::TimeoutConfig {
            // Some crates.io sparse-index documents exceed 1 MiB. Under
            // bounded parallel resolution they can take longer than the
            // generic 30s API timeout, so allow enough time for the body.
            request: Duration::from_secs(120),
            ..mgc_http::timeout::TimeoutConfig::default()
        };
        let client = mgc_http::HttpClient::with_security(&timeout, &mgc_http::TlsConfig::default())
            .expect("default crates.io HTTP/TLS policy is valid");
        Self {
            index_url: index_url.trim_end_matches('/').to_string(),
            download_base: download_base.trim_end_matches('/').to_string(),
            client,
        }
    }

    /// Build from environment.
    /// Dựng từ biến môi trường.
    pub fn from_env() -> Self {
        let index_url = std::env::var("MGC_CRATES_INDEX_URL")
            .ok()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| DEFAULT_INDEX_URL.to_string());
        let download_base = std::env::var("MGC_CRATES_DOWNLOAD_URL")
            .ok()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| DEFAULT_DOWNLOAD_BASE.to_string());
        Self::with_download_base(&index_url, &download_base)
    }

    /// Materialize a verified crate into Cargo's directory-source layout.
    /// Materialize crate đã xác minh vào layout directory-source của Cargo.
    pub fn materialize(
        &self,
        entry: &ResolvedEntry,
        bytes: &[u8],
        cargo_home: &std::path::Path,
    ) -> MgResult<PathBuf> {
        super::materialize::materialize(entry, bytes, cargo_home).map_err(mgc_types::MgError::from)
    }
}

impl Default for CratesProtocol {
    fn default() -> Self {
        Self::from_env()
    }
}

#[async_trait]
impl RegistryProtocol for CratesProtocol {
    fn matches_requirement(&self, range: &str, version: &Version) -> bool {
        super::version::cargo_matches(range, version)
    }

    async fn resolve(&self, name: &str, range: &str) -> MgResult<ResolvedEntry> {
        super::resolve::resolve(self, name, range)
            .await
            .map_err(mgc_types::MgError::from)
    }

    async fn resolve_graph(&self, name: &str, range: &str) -> MgResult<Vec<ResolvedEntry>> {
        let root = RegistryRoot {
            name: name.to_string(),
            range: range.to_string(),
            features: Vec::new(),
            default_features: true,
        };
        super::features::resolve_feature_graph(self, &[root])
            .await
            .map_err(mgc_types::MgError::from)
    }

    async fn resolve_graph_roots(
        &self,
        roots: &[(String, String)],
    ) -> MgResult<Vec<ResolvedEntry>> {
        let roots = roots
            .iter()
            .map(|(name, range)| RegistryRoot {
                name: name.clone(),
                range: range.clone(),
                features: Vec::new(),
                default_features: true,
            })
            .collect::<Vec<_>>();
        super::features::resolve_feature_graph(self, &roots)
            .await
            .map_err(mgc_types::MgError::from)
    }

    async fn resolve_graph_roots_with_options(
        &self,
        roots: &[RegistryRoot],
    ) -> MgResult<Vec<ResolvedEntry>> {
        super::features::resolve_feature_graph(self, roots)
            .await
            .map_err(mgc_types::MgError::from)
    }

    async fn download(&self, entry: &ResolvedEntry) -> MgResult<Vec<u8>> {
        super::http::get_bytes(self, &entry.artifact_url)
            .await
            .map_err(mgc_types::MgError::from)
    }
}

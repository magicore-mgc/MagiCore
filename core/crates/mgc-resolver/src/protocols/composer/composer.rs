//! Native Packagist (Composer) protocol state and public construction boundary.
//! Trạng thái protocol Packagist (Composer) native và boundary khởi tạo public.

const DEFAULT_REPOSITORY_URL: &str = "https://repo.packagist.org";

/// Native Packagist engine (Composer v2 metadata repository).
/// Engine Packagist native (repository metadata Composer v2).
#[derive(Debug, Clone)]
pub struct ComposerProtocol {
    pub(in crate::protocols::composer) repository_url: String,
    pub(in crate::protocols::composer) client: mgc_http::HttpClient,
}

impl ComposerProtocol {
    /// Build with an explicit Composer repository URL.
    /// Dựng với URL repository Composer tường minh.
    pub fn new(repository_url: &str) -> Self {
        Self {
            repository_url: repository_url.trim_end_matches('/').to_string(),
            client: mgc_http::HttpClient::default(),
        }
    }

    /// Metadata URL for one `vendor/package` pair (p2 layout).
    /// URL metadata cho một cặp `vendor/package` (layout p2).
    pub(in crate::protocols::composer) fn metadata_url(&self, name: &str) -> String {
        format!("{}/p2/{name}.json", self.repository_url)
    }

    /// Build from environment: `MGC_COMPOSER_REPOSITORY_URL`.
    /// Dựng từ môi trường: `MGC_COMPOSER_REPOSITORY_URL`.
    pub fn from_env() -> Self {
        let repository_url = std::env::var("MGC_COMPOSER_REPOSITORY_URL")
            .ok()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| DEFAULT_REPOSITORY_URL.to_string());
        Self::new(&repository_url)
    }
}

impl Default for ComposerProtocol {
    fn default() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
#[path = "test/composer_protocol/composer_protocol.rs"]
mod tests;

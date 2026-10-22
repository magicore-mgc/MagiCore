//! pub.dev protocol state and public construction boundary.
//! Trạng thái protocol pub.dev và boundary khởi tạo public.

const DEFAULT_API_URL: &str = "https://pub.dev";

/// Native pub.dev engine.
/// Engine native pub.dev.
#[derive(Debug, Clone)]
pub struct PubProtocol {
    pub(in crate::protocols::pubdev) api_url: String,
    pub(in crate::protocols::pubdev) client: mgc_http::HttpClient,
}

impl PubProtocol {
    /// Build with an explicit API URL.
    /// Dựng với API URL tường minh.
    pub fn new(api_url: &str) -> Self {
        Self {
            api_url: api_url.trim_end_matches('/').to_string(),
            client: mgc_http::HttpClient::default(),
        }
    }

    /// Build from environment: `MGC_PUB_INDEX_URL`.
    /// Dựng từ môi trường: `MGC_PUB_INDEX_URL`.
    pub fn from_env() -> Self {
        let api_url = std::env::var("MGC_PUB_INDEX_URL")
            .ok()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| DEFAULT_API_URL.to_string());
        Self::new(&api_url)
    }
}

impl Default for PubProtocol {
    fn default() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

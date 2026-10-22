//! Protocol configuration and shared HTTP client.
//! Cấu hình protocol và HTTP client dùng chung.

/// Default public Hub API endpoint.
/// Endpoint Hub công khai mặc định.
pub(super) const DEFAULT_API_URL: &str = "https://huggingface.co";

/// Hugging Face Hub model protocol client.
/// Client protocol model Hugging Face Hub.
pub struct HfHubProtocol {
    pub(super) base_url: String,
    pub(super) client: mgc_http::HttpClient,
}

impl HfHubProtocol {
    /// Build with an explicit Hub base URL.
    /// Dựng với URL gốc Hub tường minh.
    pub fn new(base_url: &str) -> Self {
        let token = std::env::var("MGC_HF_TOKEN").ok().filter(|v| !v.is_empty());
        // Bearer auth rides the shared client instead of per-request builders.
        // Auth Bearer đi theo client dùng chung thay vì từng request.
        let base_client = mgc_http::HttpClient::default();
        let client = match &token {
            Some(t) => base_client.with_auth("authorization", format!("Bearer {t}")),
            None => base_client,
        };
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
        }
    }

    /// Build from environment variables.
    /// Dựng từ biến môi trường.
    pub fn from_env() -> Self {
        let base_url = std::env::var("MGC_HF_API_URL")
            .ok()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| DEFAULT_API_URL.to_string());
        Self::new(&base_url)
    }
}

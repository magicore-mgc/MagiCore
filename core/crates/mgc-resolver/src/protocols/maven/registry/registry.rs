//! Maven repository configuration and stable protocol type.
//! Cấu hình Maven repository và kiểu protocol ổn định.

const DEFAULT_REPO_URL: &str = "https://repo.maven.apache.org/maven2";

/// Native Maven repository engine.
/// Engine native repository Maven.
#[derive(Debug, Clone)]
pub struct MavenProtocol {
    // Sibling registry owners need these fields; they remain private outside this folder.
    // Các owner sibling cần hai field này; bên ngoài folder vẫn không truy cập được.
    pub(super) repo_url: String,
    pub(super) client: mgc_http::HttpClient,
}

impl MavenProtocol {
    /// Build with an explicit repository base URL.
    /// Dựng với URL gốc repository tường minh.
    pub fn new(repo_url: &str) -> Self {
        Self {
            repo_url: repo_url.trim_end_matches('/').to_string(),
            client: mgc_http::HttpClient::default(),
        }
    }

    /// Build from environment: `MGC_MAVEN_REPO_URL`.
    /// Dựng từ môi trường: `MGC_MAVEN_REPO_URL`.
    pub fn from_env() -> Self {
        let repo_url = std::env::var("MGC_MAVEN_REPO_URL")
            .ok()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| DEFAULT_REPO_URL.to_string());
        Self::new(&repo_url)
    }
}

impl Default for MavenProtocol {
    fn default() -> Self {
        Self::from_env()
    }
}

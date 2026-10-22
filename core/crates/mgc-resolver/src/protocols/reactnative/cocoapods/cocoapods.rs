//! CocoaPods protocol state and stable public operations.
//! State protocol CocoaPods và các operation công khai ổn định.

use mgc_http::HttpClient;
use mgc_types::MgResult;
use serde_json::Value;

const DEFAULT_CDN_URL: &str = "https://cdn.cocoapods.org";

/// A verified pod spec: SHA-1 of the exact downloaded specification bytes.
/// Spec pod đã xác minh: SHA-1 của đúng byte specification đã tải.
#[derive(Debug, Clone)]
pub struct VerifiedPodSpec {
    pub sha1: String,
    pub spec: Value,
}

/// CocoaPods CDN client with fail-closed SHA-1 verification.
/// Client CDN CocoaPods với xác minh SHA-1 fail-closed.
#[derive(Debug, Clone)]
pub struct CocoaPodsProtocol {
    pub(super) cdn: String,
    pub(super) client: HttpClient,
}

impl CocoaPodsProtocol {
    pub fn with_cdn(cdn: &str) -> Self {
        Self {
            cdn: cdn.trim_end_matches('/').to_string(),
            client: HttpClient::default(),
        }
    }

    /// Build from `MGC_COCOAPODS_CDN_URL`, falling back to the public CDN.
    /// Dựng từ `MGC_COCOAPODS_CDN_URL`, mặc định về CDN công khai.
    pub fn from_env() -> Self {
        let cdn = std::env::var("MGC_COCOAPODS_CDN_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_CDN_URL.to_string());
        Self::with_cdn(&cdn)
    }

    pub fn cdn(&self) -> &str {
        &self.cdn
    }

    /// Fetch one spec and verify its digest against the lockfile checksum.
    /// Tải một spec và xác minh digest theo checksum trong lockfile.
    pub async fn fetch_verified_spec(
        &self,
        name: &str,
        version: &str,
        expected_sha1: &str,
    ) -> MgResult<VerifiedPodSpec> {
        super::client::fetch_verified_spec(self, name, version, expected_sha1)
            .await
            .map_err(super::client::CocoaPodsClientError::into_mg_error)
    }

    /// Return declared podspec edges as `(name, requirement)` pairs.
    /// Trả cạnh khai báo trong podspec dạng cặp `(name, requirement)`.
    pub fn spec_dependencies(spec: &Value) -> Vec<(String, String)> {
        super::dependencies::spec_dependencies(spec)
    }
}

#[cfg(test)]
#[path = "test/cocoapods.rs"]
mod tests;

//! Public PyPI client and RegistryClient coordinator.
//! Client PyPI công khai và điều phối RegistryClient.

use super::{metadata, metadata_request, package_download};
use crate::native::{PackageMetadata, RegistryClient};
use mgc_types::{MgError, MgResult, PackageId, PackageName, Version};

/// PyPI HTTP client.
/// Client HTTP PyPI.
pub struct PyPiClient {
    registry_url: String,
    http_client: reqwest::Client,
}

impl PyPiClient {
    /// Create a client for the default PyPI registry.
    /// Tạo client dùng registry PyPI mặc định.
    pub fn new() -> Self {
        Self {
            registry_url: "https://pypi.org".to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    /// Create a client for a caller-selected registry URL.
    /// Tạo client theo registry URL do caller chọn.
    pub fn with_registry(registry_url: String) -> Self {
        Self {
            registry_url,
            http_client: reqwest::Client::new(),
        }
    }

    /// Construct the existing PyPI JSON API route for a package.
    /// Tạo route JSON API PyPI hiện hành cho package.
    fn api_url(&self, name: &PackageName) -> String {
        format!("{}/pypi/{}/json", self.registry_url, name.as_str())
    }
}

#[async_trait::async_trait]
impl RegistryClient for PyPiClient {
    async fn fetch_metadata(&self, name: &PackageName) -> MgResult<PackageMetadata> {
        let url = self.api_url(name);
        let response = metadata_request::fetch(
            &self.http_client,
            &url,
            metadata_request::StatusPolicy::RequireSuccess,
        )
        .await?;
        metadata::project(name, response).map_err(MgError::from)
    }

    async fn download_package(&self, package_id: &PackageId) -> MgResult<Vec<u8>> {
        let url = self.api_url(package_id.name());
        package_download::download(&self.http_client, &url, package_id)
            .await
            .map_err(MgError::from)
    }

    async fn list_versions(&self, name: &PackageName) -> MgResult<Vec<Version>> {
        let metadata = self.fetch_metadata(name).await?;
        Ok(metadata.versions)
    }
}

impl Default for PyPiClient {
    fn default() -> Self {
        Self::new()
    }
}

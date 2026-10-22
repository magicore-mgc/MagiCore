//! Typed internal failures for PyPI HTTP operations.
//! Lỗi nội bộ có kiểu cho thao tác HTTP PyPI.

use mgc_types::{MgError, Version};

/// Keeps causes typed until the legacy MgError boundary.
/// Giữ cause có kiểu đến ranh giới MgError tương thích.
#[derive(Debug, thiserror::Error)]
pub(in crate::native::pypi_client) enum PyPiClientError {
    #[error("failed to fetch package metadata: {0}")]
    MetadataRequest(#[source] reqwest::Error),
    #[error("package not found or registry error: {0}")]
    RegistryStatus(reqwest::StatusCode),
    #[error("failed to parse metadata: {0}")]
    MetadataParse(#[source] reqwest::Error),
    #[error("invalid latest version: {0}")]
    InvalidLatestVersion(#[source] MgError),
    #[error("version {0} not found")]
    VersionNotFound(Version),
    #[error("no downloadable files found")]
    NoDownloadableFiles,
    #[error("failed to download package: {0}")]
    PackageRequest(#[source] reqwest::Error),
    #[error("download failed: {0}")]
    PackageStatus(reqwest::StatusCode),
    #[error("failed to read package data: {0}")]
    PackageBody(#[source] reqwest::Error),
}

impl From<PyPiClientError> for MgError {
    fn from(error: PyPiClientError) -> Self {
        Self::Other(error.to_string())
    }
}

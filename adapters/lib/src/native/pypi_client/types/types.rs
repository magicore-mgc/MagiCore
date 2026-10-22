//! Private DTOs matching the PyPI JSON API response shape.
//! DTO nội bộ khớp cấu trúc response JSON API PyPI.

use serde::Deserialize;

/// Package-level information from the JSON API.
/// Thông tin package từ JSON API.
#[derive(Debug, Deserialize)]
pub(in crate::native::pypi_client) struct PyPiMetadata {
    pub(in crate::native::pypi_client) info: PyPiInfo,
    pub(in crate::native::pypi_client) releases:
        std::collections::HashMap<String, Vec<PyPiRelease>>,
}

/// Descriptive fields from the JSON API.
/// Các trường mô tả từ JSON API.
#[derive(Debug, Deserialize)]
pub(in crate::native::pypi_client) struct PyPiInfo {
    pub(in crate::native::pypi_client) version: String,
    pub(in crate::native::pypi_client) summary: String,
    pub(in crate::native::pypi_client) home_page: Option<String>,
    pub(in crate::native::pypi_client) project_url: Option<String>,
}

/// One downloadable file from a release.
/// Một file có thể tải trong một release.
#[derive(Debug, Deserialize)]
pub(in crate::native::pypi_client) struct PyPiRelease {
    pub(in crate::native::pypi_client) url: String,
    pub(in crate::native::pypi_client) packagetype: String,
}

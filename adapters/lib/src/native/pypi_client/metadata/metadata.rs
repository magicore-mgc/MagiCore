//! Converts the existing PyPI JSON response into adapter metadata.
//! Chuyển response JSON PyPI hiện tại thành metadata của adapter.

use crate::native::PackageMetadata;
use mgc_types::{PackageName, Version};

use super::super::{error::PyPiClientError, types::PyPiMetadata};

pub(in crate::native::pypi_client) fn project(
    name: &PackageName,
    data: PyPiMetadata,
) -> Result<PackageMetadata, PyPiClientError> {
    let mut versions = Vec::new();
    for version_text in data.releases.keys() {
        if let Ok(version) = Version::parse(version_text) {
            versions.push(version);
        }
    }
    versions.sort();

    let latest =
        Version::parse(&data.info.version).map_err(PyPiClientError::InvalidLatestVersion)?;

    Ok(PackageMetadata {
        name: name.clone(),
        versions,
        latest,
        description: Some(data.info.summary),
        homepage: data.info.home_page,
        repository: data.info.project_url,
    })
}

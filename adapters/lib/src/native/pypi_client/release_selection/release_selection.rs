//! Selects the existing preferred artifact for one PyPI release.
//! Chọn artifact ưu tiên hiện hành cho một release PyPI.

use mgc_types::PackageId;

use super::super::{
    error::PyPiClientError,
    types::{PyPiMetadata, PyPiRelease},
};

pub(in crate::native::pypi_client) fn select<'a>(
    metadata: &'a PyPiMetadata,
    package_id: &PackageId,
) -> Result<&'a PyPiRelease, PyPiClientError> {
    let version = package_id.version();
    let releases = metadata
        .releases
        .get(&version.to_string())
        .ok_or_else(|| PyPiClientError::VersionNotFound(version.clone()))?;

    releases
        .iter()
        .find(|release| release.packagetype == "bdist_wheel")
        .or_else(|| {
            releases
                .iter()
                .find(|release| release.packagetype == "sdist")
        })
        .ok_or(PyPiClientError::NoDownloadableFiles)
}

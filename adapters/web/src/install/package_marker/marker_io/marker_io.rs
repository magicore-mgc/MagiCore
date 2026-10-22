//! Read/write marker metadata and apply its fast identity checks.
//! Đọc/ghi metadata marker và kiểm tra nhanh định danh.

use std::path::{Path, PathBuf};

use mgc_types::{MgError, MgResult};

use crate::cache::ExtractedPackageMarker;
use crate::manifest::atomic_write;

pub fn extracted_package_marker_path(root: &Path) -> PathBuf {
    root.join(".magicore-package-root.json")
}

pub fn extracted_marker_matches_fast(
    marker: &ExtractedPackageMarker,
    expected: &ExtractedPackageMarker,
) -> bool {
    marker.schema_version == 3
        && marker.name == expected.name
        && marker.version == expected.version
        && marker.integrity == expected.integrity
        && marker.tarball_sha256 == expected.tarball_sha256
}

pub fn extracted_marker_has_content_signature(marker: &ExtractedPackageMarker) -> bool {
    marker.file_count > 0 && marker.unpacked_size > 0 && !marker.file_tree_sha256.trim().is_empty()
}

pub fn read_extracted_package_marker(root: &Path) -> MgResult<Option<ExtractedPackageMarker>> {
    let path = extracted_package_marker_path(root);
    if !path.exists() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(&path).map_err(|err| {
        MgError::Other(format!(
            "failed to read extracted package marker '{}': {}",
            path.display(),
            err
        ))
    })?;
    let marker = serde_json::from_str(&contents).map_err(|err| {
        MgError::Other(format!(
            "failed to parse extracted package marker '{}': {}",
            path.display(),
            err
        ))
    })?;
    Ok(Some(marker))
}

pub fn write_extracted_package_marker(
    root: &Path,
    marker: &ExtractedPackageMarker,
) -> MgResult<()> {
    let path = extracted_package_marker_path(root);
    let payload = serde_json::to_vec_pretty(marker).map_err(|err| {
        MgError::Other(format!(
            "failed to serialize extracted package marker '{}': {}",
            path.display(),
            err
        ))
    })?;
    atomic_write(&path, &payload)?;
    Ok(())
}

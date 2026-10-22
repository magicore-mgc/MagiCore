//! Manifest selection and format-priority coordinator.
//! Điều phối lựa chọn manifest và thứ tự ưu tiên định dạng.

use super::{cargo_manifest, error::ManifestReadError, package_json, pyproject};
use crate::graph::types::WorkspacePackageManifest;
use std::path::Path;

/// Read a package manifest using the existing Node → Rust → Python priority.
/// Đọc manifest theo thứ tự ưu tiên Node → Rust → Python hiện có.
pub fn read_package_manifest(path: &Path) -> anyhow::Result<Option<WorkspacePackageManifest>> {
    let package_json_path = path.join("package.json");
    if package_json_path.exists() {
        return package_json::read_package_json(&package_json_path)
            .map(Some)
            .map_err(ManifestReadError::into_anyhow);
    }

    let cargo_path = path.join("Cargo.toml");
    if cargo_path.exists()
        && let Some(manifest) = cargo_manifest::read_cargo_manifest(&cargo_path)
            .map_err(ManifestReadError::into_anyhow)?
    {
        return Ok(Some(manifest));
    }

    let pyproject_path = path.join("pyproject.toml");
    if pyproject_path.exists()
        && let Some(manifest) = pyproject::read_pyproject_manifest(&pyproject_path)
            .map_err(ManifestReadError::into_anyhow)?
    {
        return Ok(Some(manifest));
    }

    Ok(None)
}

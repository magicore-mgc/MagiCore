//! Safely read the installed package manifest and extract lifecycle hooks.
//! Đọc manifest package đã cài an toàn và lấy các hook lifecycle.

use super::error::{LifecycleError, LifecycleResult};
use super::types::{PackageManifest, PackageScripts};
use std::path::Path;

/// Read lifecycle scripts from a no-follow, size-bounded manifest snapshot.
/// Policy inspection and execution must share this exact parsed value.
/// Đọc snapshot không theo symlink; policy và thực thi phải dùng cùng dữ liệu.
pub(crate) fn load_package_scripts(pkg_dir: &Path) -> LifecycleResult<PackageScripts> {
    let package_json = pkg_dir.join("package.json");
    let contents =
        mgc_config::project::read_regular_project_text(&package_json, "installed package manifest")
            .map_err(|source| LifecycleError::ManifestRead {
                path: package_json.display().to_string(),
                source,
            })?;
    let Some(contents) = contents else {
        return Ok(PackageScripts::default());
    };
    let manifest: PackageManifest =
        serde_json::from_str(&contents).map_err(|source| LifecycleError::ManifestParse {
            path: package_json.display().to_string(),
            source,
        })?;
    Ok(manifest.scripts)
}

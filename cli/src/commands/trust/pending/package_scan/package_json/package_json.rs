//! Validates lifecycle-script fields and records the package identity.
//! Kiểm tra field lifecycle script và ghi nhận danh tính package.

use std::collections::HashSet;
use std::path::Path;

use super::super::super::{PendingResult, TrustPendingError};
use super::super::bounded_manifest::read_bounded_package_json;

pub(in crate::commands::trust::pending::package_scan) fn check_package_json(
    pkg_dir: &Path,
    name: &str,
    out: &mut HashSet<(String, String)>,
) -> PendingResult<()> {
    let manifest_path = pkg_dir.join("package.json");
    let Some(contents) = read_bounded_package_json(&manifest_path)? else {
        return Ok(());
    };
    let manifest = serde_json::from_slice::<serde_json::Value>(&contents)
        .map_err(|error| TrustPendingError::scan_failed(&manifest_path, error))?;
    let manifest = manifest.as_object().ok_or_else(|| {
        TrustPendingError::manifest_invalid(&manifest_path, "root must be a JSON object")
    })?;
    let Some(scripts_value) = manifest.get("scripts") else {
        return Ok(());
    };
    let scripts = scripts_value.as_object().ok_or_else(|| {
        TrustPendingError::manifest_invalid(&manifest_path, "'scripts' must be a JSON object")
    })?;
    for lifecycle_hook in ["preinstall", "install", "postinstall"] {
        if scripts
            .get(lifecycle_hook)
            .is_some_and(|value| !value.is_string())
        {
            return Err(TrustPendingError::manifest_invalid(
                &manifest_path,
                "lifecycle script values must be strings",
            ));
        }
    }
    let has_scripts = ["preinstall", "install", "postinstall"]
        .iter()
        .any(|hook| scripts.contains_key(*hook));
    if !has_scripts {
        return Ok(());
    }
    // Prefer the manifest's own `name` (directory layouts like
    // `.store/name@version` would otherwise mislabel the package and
    // break policy matching).
    // (Ưu tiên trường name trong manifest hơn tên thư mục.)
    let name = manifest
        .get("name")
        .and_then(|v| v.as_str())
        .filter(|n| !n.is_empty())
        .unwrap_or(name)
        .to_string();
    let version = manifest
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    out.insert((name, version));
    Ok(())
}

#[cfg(test)]
#[path = "test/package_json.rs"]
mod tests;

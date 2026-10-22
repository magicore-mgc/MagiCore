//! Reads the requested package's own manifest identity and executable declarations.
//! Đọc identity và khai báo executable từ manifest của chính package được yêu cầu.

use anyhow::Result;
use std::path::{Path, PathBuf};

// Read only the installed requested package's executable declarations.
// Đọc bin của đúng package sau native install, không quét bin dependency.
pub(crate) fn requested_package_bins(root: &Path, package: &str) -> Result<Vec<(String, PathBuf)>> {
    let text = mgc_config::project::read_regular_project_text(
        &root.join("package.json"),
        "installed package manifest",
    )?
    .ok_or_else(super::registry_package_invalid_bins)?;
    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|_| super::registry_package_invalid_bins())?;
    if value.get("name").and_then(serde_json::Value::as_str) != Some(package) {
        return Err(super::registry_package_invalid_bins());
    }
    match value.get("bin") {
        None | Some(serde_json::Value::Null) => Ok(Vec::new()),
        Some(serde_json::Value::String(target)) => Ok(vec![(
            package.rsplit('/').next().unwrap_or(package).to_string(),
            PathBuf::from(target),
        )]),
        Some(serde_json::Value::Object(entries)) => entries
            .iter()
            .map(|(name, target)| {
                target
                    .as_str()
                    .map(|target| (name.clone(), PathBuf::from(target)))
                    .ok_or_else(super::registry_package_invalid_bins)
            })
            .collect(),
        _ => Err(super::registry_package_invalid_bins()),
    }
}

pub(crate) fn requested_package_version(root: &Path, package: &str) -> Result<String> {
    let text = mgc_config::project::read_regular_project_text(
        &root.join("package.json"),
        "installed package manifest",
    )?
    .ok_or_else(super::registry_package_invalid_bins)?;
    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|_| super::registry_package_invalid_bins())?;
    if value.get("name").and_then(serde_json::Value::as_str) != Some(package) {
        return Err(super::registry_package_invalid_bins());
    }
    let version = value
        .get("version")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(super::registry_package_invalid_bins)?;
    mgc_types::Version::parse(version).map_err(|_| super::registry_package_invalid_bins())?;
    Ok(version.to_string())
}
#[cfg(test)]
#[path = "test/installed_metadata.rs"]
mod tests;

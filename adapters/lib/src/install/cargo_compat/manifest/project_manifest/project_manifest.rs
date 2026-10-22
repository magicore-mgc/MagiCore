//! Parse the supported single-package Cargo.toml subset.
//! Parse tập Cargo.toml single-package được hỗ trợ.

use super::super::super::CargoProjectManifest;
use super::super::error::CargoManifestError;

/// Parse a project manifest and reject unsupported Cargo workspace features.
/// Parse manifest dự án và chặn các tính năng Cargo workspace chưa hỗ trợ.
pub(in crate::install) fn read_project_manifest(
    project_root: &std::path::Path,
) -> Result<CargoProjectManifest, CargoManifestError> {
    let manifest_path = project_root.join("Cargo.toml");
    let manifest = mgc_adapter_base::project_file::read_regular_text(&manifest_path, "Cargo.toml")
        .map_err(CargoManifestError::ProjectFile)?;
    let manifest: toml::Value = toml::from_str(&manifest).map_err(CargoManifestError::TomlParse)?;
    let package_table = manifest
        .get("package")
        .and_then(toml::Value::as_table)
        .ok_or(CargoManifestError::PackageManifestRequired)?;
    if package_table.contains_key("workspace") {
        return Err(CargoManifestError::WorkspaceInheritedMetadataUnsupported);
    }
    if let Some(workspace) = manifest.get("workspace").and_then(toml::Value::as_table)
        && ["members", "default-members"].iter().any(|key| {
            workspace
                .get(*key)
                .and_then(toml::Value::as_array)
                .is_some_and(|members| !members.is_empty())
        })
    {
        return Err(CargoManifestError::WorkspaceUnsupported);
    }
    for section in [
        "dev-dependencies",
        "build-dependencies",
        "target",
        "patch",
        "replace",
    ] {
        if manifest
            .get(section)
            .is_some_and(|value| value.as_table().is_none_or(|table| !table.is_empty()))
        {
            return Err(CargoManifestError::UnsupportedSection { section });
        }
    }
    if manifest
        .get("features")
        .and_then(toml::Value::as_table)
        .is_some_and(|features| !features.is_empty())
    {
        return Err(CargoManifestError::CustomRootFeaturesUnsupported);
    }
    let root_name = required_string(package_table, "name", "Cargo.toml package.name")?;
    let root_version = required_string(package_table, "version", "Cargo.toml package.version")?;
    mgc_types::Version::parse(root_version).map_err(CargoManifestError::InvalidVersion)?;

    let root_name = root_name.to_owned();
    let root_version = root_version.to_owned();
    Ok(CargoProjectManifest {
        document: manifest,
        root_name,
        root_version,
    })
}

/// Require a non-empty string field while preserving its existing diagnostic.
/// Bắt buộc trường chuỗi không rỗng và giữ nguyên thông báo lỗi hiện có.
pub(in crate::install) fn required_string<'a>(
    table: &'a toml::value::Table,
    key: &str,
    label: &str,
) -> Result<&'a str, CargoManifestError> {
    table
        .get(key)
        .and_then(toml::Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| CargoManifestError::RequiredString {
            label: label.to_string(),
        })
}

#[cfg(test)]
#[path = "test/project_manifest.rs"]
mod tests;

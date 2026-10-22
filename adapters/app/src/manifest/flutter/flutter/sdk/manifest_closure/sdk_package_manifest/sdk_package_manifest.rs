//! Load and validate dependency metadata from one Flutter SDK package.
//! Tải và kiểm tra metadata dependency của một package Flutter SDK.

use crate::manifest::flutter::{FlutterError, FlutterResult};
use std::path::Path;

/// Return a package's dependency mapping, or `None` when it declares none.
/// Trả mapping dependency hoặc `None` nếu package không khai báo.
pub(in crate::manifest::flutter::flutter::sdk::manifest_closure) fn flutter_sdk_package_dependencies(
    canonical_sdk_root: &Path,
    name: &str,
) -> FlutterResult<Option<serde_yaml::Mapping>> {
    let package_root =
        super::super::super::package_root::flutter_sdk_package_root(canonical_sdk_root, name)?;
    let package_manifest_path = package_root.join("pubspec.yaml");
    let metadata = std::fs::symlink_metadata(&package_manifest_path).map_err(|error| {
        FlutterError::Other(format!(
            "inspect Flutter SDK package manifest '{}': {error}",
            package_manifest_path.display()
        ))
    })?;
    if !metadata.file_type().is_file() {
        return Err(FlutterError::Integrity(format!(
            "Flutter SDK package '{name}' has a linked or non-regular pubspec.yaml"
        )));
    }
    let content = mgc_adapter_base::project_file::read_regular_text(
        &package_manifest_path,
        &format!("Flutter SDK package '{name}' pubspec.yaml"),
    )?;
    let package_yaml: serde_yaml::Value = serde_yaml::from_str(&content).map_err(|error| {
        FlutterError::Other(format!(
            "parse Flutter SDK package '{name}' manifest: {error}"
        ))
    })?;
    if package_yaml.get("name").and_then(serde_yaml::Value::as_str) != Some(name) {
        return Err(FlutterError::Integrity(format!(
            "Flutter SDK package identity mismatch for '{name}'"
        )));
    }
    if package_yaml
        .get("dependency_overrides")
        .is_some_and(|overrides| !overrides.is_null())
        || package_yaml
            .get("workspace")
            .is_some_and(|workspace| !workspace.is_null())
    {
        return Err(FlutterError::Unsupported {
            capability: "Flutter SDK package override/workspace semantics",
            guidance: format!(
                "Flutter SDK package '{name}' declares dependency_overrides or workspace metadata that MagiCore cannot resolve natively"
            ),
        });
    }
    match package_yaml.get("dependencies") {
        None | Some(serde_yaml::Value::Null) => Ok(None),
        Some(serde_yaml::Value::Mapping(dependencies)) => Ok(Some(dependencies.clone())),
        Some(_) => Err(FlutterError::Integrity(format!(
            "Flutter SDK package '{name}' dependencies must be a mapping"
        ))),
    }
}

//! Read a canonical Flutter SDK package manifest and verify its identity.
//! Đọc manifest package SDK Flutter canonical và xác minh identity.

use super::super::FlutterPackageGraphError;
use std::path::Path;

pub(super) fn read_sdk_manifest(
    sdk_root: &Path,
    name: &str,
) -> Result<serde_yaml::Value, FlutterPackageGraphError> {
    let package_root = crate::manifest::flutter::flutter_sdk_package_root(sdk_root, name)?;
    let manifest_path = package_root.join("pubspec.yaml");
    let manifest_text = std::fs::read_to_string(&manifest_path).map_err(|source| {
        FlutterPackageGraphError::ReadSdkManifest {
            path: manifest_path.display().to_string(),
            source,
        }
    })?;
    let manifest: serde_yaml::Value = serde_yaml::from_str(&manifest_text).map_err(|source| {
        FlutterPackageGraphError::ParseSdkManifest {
            path: manifest_path.display().to_string(),
            source,
        }
    })?;
    let actual_name = manifest.get("name").and_then(serde_yaml::Value::as_str);
    if actual_name != Some(name) {
        return Err(FlutterPackageGraphError::SdkIdentityMismatch(
            name.to_string(),
        ));
    }
    Ok(manifest)
}

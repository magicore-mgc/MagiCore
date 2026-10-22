//! Builds and validates entries for Flutter SDK packages.
//! Dựng và kiểm tra entry của package Flutter SDK.

use std::collections::BTreeSet;
use std::path::Path;

use super::super::super::types::FlutterPackageConfigEntry;
use super::super::super::version_floor::dart_language_version_floor;
use super::error::FlutterSdkEntriesError;

pub(super) fn build_sdk_entries_inner(
    sdk_packages: &[String],
    sdk_root: Option<&Path>,
    seen: &mut BTreeSet<String>,
) -> Result<Vec<FlutterPackageConfigEntry>, FlutterSdkEntriesError> {
    let mut entries = Vec::with_capacity(sdk_packages.len());
    if !sdk_packages.is_empty() {
        let sdk_root = sdk_root.ok_or(FlutterSdkEntriesError::MissingSdkRoot)?;
        for name in sdk_packages {
            if !seen.insert(name.clone()) {
                return Err(FlutterSdkEntriesError::RegistryConflict(name.clone()));
            }
            let canonical_package_root =
                crate::manifest::flutter::flutter_sdk_package_root(sdk_root, name)?;
            let package_manifest_path = canonical_package_root.join("pubspec.yaml");
            let package_manifest_meta =
                std::fs::symlink_metadata(&package_manifest_path).map_err(|source| {
                    FlutterSdkEntriesError::InspectManifest {
                        path: package_manifest_path.display().to_string(),
                        source,
                    }
                })?;
            if !package_manifest_meta.file_type().is_file() {
                return Err(FlutterSdkEntriesError::NonRegularManifest(name.clone()));
            }
            let manifest_text =
                std::fs::read_to_string(&package_manifest_path).map_err(|source| {
                    FlutterSdkEntriesError::ReadManifest {
                        path: package_manifest_path.display().to_string(),
                        source,
                    }
                })?;
            let manifest: serde_yaml::Value =
                serde_yaml::from_str(&manifest_text).map_err(|source| {
                    FlutterSdkEntriesError::ParseManifest {
                        path: package_manifest_path.display().to_string(),
                        source,
                    }
                })?;
            if manifest.get("name").and_then(serde_yaml::Value::as_str) != Some(name.as_str()) {
                return Err(FlutterSdkEntriesError::IdentityMismatch(name.clone()));
            }
            let lib_dir = canonical_package_root.join("lib");
            if !std::fs::symlink_metadata(&lib_dir).is_ok_and(|meta| meta.is_dir()) {
                return Err(FlutterSdkEntriesError::MissingRegularLib(name.clone()));
            }
            let language_version = manifest
                .get("environment")
                .and_then(|value| value.get("sdk"))
                .and_then(serde_yaml::Value::as_str)
                .and_then(dart_language_version_floor);
            let root_uri = url::Url::from_directory_path(&canonical_package_root)
                .map_err(|_| FlutterSdkEntriesError::EncodePackageUri)?
                .to_string();
            entries.push(FlutterPackageConfigEntry {
                name: name.clone(),
                root_uri,
                package_uri: "lib/",
                language_version,
            });
        }
    }
    Ok(entries)
}

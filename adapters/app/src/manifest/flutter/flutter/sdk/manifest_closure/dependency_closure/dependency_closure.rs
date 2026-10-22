//! Traverse Flutter SDK package edges and extend the native pub manifest.
//! Duyệt cạnh package Flutter SDK và mở rộng manifest pub native.

use crate::manifest::flutter::{FlutterError, FlutterResult};
use mgc_types::{Manifest, VersionRange};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::Path;

/// Read the reachable Flutter SDK package set and merge its registry dependencies.
/// Đọc tập package Flutter SDK có thể tới được và gộp dependency registry của chúng.
pub(in crate::manifest::flutter::flutter::sdk) fn read_flutter_sdk_dependency_closure(
    project_root: &Path,
    sdk_root: &Path,
    base_manifest: &Manifest,
    declared: Vec<(String, bool)>,
) -> FlutterResult<(Vec<String>, Manifest)> {
    let canonical_sdk_root = sdk_root
        .canonicalize()
        .map_err(|error| FlutterError::Other(format!("canonicalize Flutter SDK root: {error}")))?;
    let project_dependencies =
        super::super::project_dependencies::project_dependency_groups(project_root)?;
    let mut manifest = base_manifest.clone();
    let mut pending: VecDeque<(String, bool)> = declared.into();
    let mut package_dev_state: BTreeMap<String, bool> = BTreeMap::new();
    let mut sdk_names = BTreeSet::new();
    let mut registry_ranges = BTreeMap::<String, (VersionRange, bool)>::new();
    while let Some((name, dev)) = pending.pop_front() {
        super::super::super::package_root::validate_flutter_sdk_package_name(&name)?;
        if package_dev_state
            .get(&name)
            .is_some_and(|previous_dev| !*previous_dev || *previous_dev == dev)
        {
            continue;
        }
        package_dev_state.insert(name.clone(), dev);
        sdk_names.insert(name.clone());
        let Some(dependencies) =
            super::super::sdk_package_manifest::flutter_sdk_package_dependencies(
                &canonical_sdk_root,
                &name,
            )?
        else {
            continue;
        };
        for (dependency_name, value) in &dependencies {
            let dependency_name = dependency_name.as_str().ok_or_else(|| {
                FlutterError::Integrity("Flutter SDK dependency name must be a string".to_string())
            })?;
            if let Some(fields) = value.as_mapping() {
                let sdk = fields
                    .get(serde_yaml::Value::String("sdk".to_string()))
                    .and_then(serde_yaml::Value::as_str)
                    .ok_or_else(|| FlutterError::Unsupported {
                        capability: "Flutter SDK package dependency source",
                        guidance: format!("Flutter SDK package '{name}' uses an unsupported source for '{dependency_name}'"),
                    })?;
                if fields.len() != 1 || !matches!(sdk, "flutter" | "dart") {
                    return Err(FlutterError::Unsupported {
                        capability: "Flutter SDK package dependency source",
                        guidance: format!(
                            "Flutter SDK package '{name}' uses unsupported SDK source '{sdk}' for '{dependency_name}'"
                        ),
                    });
                }
                if sdk == "flutter" {
                    pending.push_back((dependency_name.to_string(), dev));
                }
                continue;
            }
            super::super::registry_dependency::merge_registry_dependency(
                &name,
                dependency_name,
                value,
                dev,
                &project_dependencies,
                &mut registry_ranges,
                &mut manifest,
            )?;
        }
    }
    Ok((sdk_names.into_iter().collect(), manifest))
}

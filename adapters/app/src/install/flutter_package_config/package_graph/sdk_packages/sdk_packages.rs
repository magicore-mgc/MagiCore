//! Build graph entries for selected Flutter SDK packages.
//! Dựng entry graph cho package Flutter SDK được chọn.

use super::super::super::types::FlutterPackageGraphEntry;
use super::super::FlutterPackageGraphError;
use super::sdk_dependencies::sdk_dependency_names;
use super::sdk_manifest::read_sdk_manifest;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub(in crate::install::flutter_package_config::package_graph) fn read_sdk_package_entries(
    sdk_root: &Path,
    sdk_packages: &[String],
    root_name: &str,
    existing_names: &BTreeSet<String>,
) -> Result<BTreeMap<String, FlutterPackageGraphEntry>, FlutterPackageGraphError> {
    let mut entries = BTreeMap::new();
    for name in sdk_packages {
        if name == root_name || existing_names.contains(name) || entries.contains_key(name) {
            return Err(FlutterPackageGraphError::DuplicatePackage(name.clone()));
        }
        let manifest = read_sdk_manifest(sdk_root, name)?;
        let dependencies = sdk_dependency_names(name, &manifest)?;
        entries.insert(
            name.clone(),
            FlutterPackageGraphEntry {
                name: name.clone(),
                version: manifest
                    .get("version")
                    .and_then(serde_yaml::Value::as_str)
                    .unwrap_or("0.0.0")
                    .to_string(),
                dependencies,
                dev_dependencies: None,
            },
        );
    }
    Ok(entries)
}

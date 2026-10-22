//! Convert MGC-resolved packages into name-keyed Dart graph entries.
//! Chuyển package MGC đã resolve thành entry Dart graph theo name.

use mgc_types::ResolvedGraph;
use std::collections::BTreeMap;

use super::super::super::types::FlutterPackageGraphEntry;
use super::super::FlutterPackageGraphError;

pub(in crate::install::flutter_package_config::package_graph) fn resolved_package_entries(
    graph: &ResolvedGraph,
    root_name: &str,
) -> Result<BTreeMap<String, FlutterPackageGraphEntry>, FlutterPackageGraphError> {
    let mut entries = BTreeMap::<String, FlutterPackageGraphEntry>::new();
    for package in &graph.packages {
        let name = package.id.name_str().to_string();
        if name == root_name {
            return Err(FlutterPackageGraphError::ResolvedGraphContainsRoot(
                root_name.to_string(),
            ));
        }
        if entries.contains_key(&name) {
            return Err(FlutterPackageGraphError::DuplicatePackage(name));
        }
        let mut dependencies = package
            .deps
            .iter()
            .map(|dependency| dependency.name_str().to_string())
            .collect::<Vec<_>>();
        dependencies.sort();
        dependencies.dedup();
        entries.insert(
            name.clone(),
            FlutterPackageGraphEntry {
                name,
                version: package.id.version().to_string(),
                dependencies,
                dev_dependencies: None,
            },
        );
    }
    Ok(entries)
}

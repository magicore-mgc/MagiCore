//! Coordinate the typed Dart package-graph owners.
//! Điều phối các owner typed của Dart package graph.

use mgc_types::ResolvedGraph;
use std::path::Path;

use super::super::types::{FlutterPackageGraph, FlutterPackageGraphEntry};
use super::FlutterPackageGraphError;
use super::edge_validation::validate_package_edges;
use super::resolved_packages::resolved_package_entries;
use super::root_manifest::read_root_manifest;
use super::sdk_packages::read_sdk_package_entries;

pub(super) fn build_flutter_package_graph_inner(
    graph: &ResolvedGraph,
    project_root: &Path,
    sdk_root: Option<&Path>,
    sdk_packages: &[String],
) -> Result<FlutterPackageGraph, FlutterPackageGraphError> {
    let root = read_root_manifest(project_root)?;
    let mut entries = resolved_package_entries(graph, &root.name)?;

    if !sdk_packages.is_empty() {
        let sdk_root = sdk_root.ok_or(FlutterPackageGraphError::MissingSdkRoot)?;
        let existing_names = entries.keys().cloned().collect();
        let sdk_entries =
            read_sdk_package_entries(sdk_root, sdk_packages, &root.name, &existing_names)?;
        entries.extend(sdk_entries);
    }

    validate_package_edges(&entries, &root.dependencies, &root.dev_dependencies)?;

    let mut packages = vec![FlutterPackageGraphEntry {
        name: root.name.clone(),
        version: root.version,
        dependencies: root.dependencies,
        dev_dependencies: Some(root.dev_dependencies),
    }];
    packages.extend(entries.into_values());
    Ok(FlutterPackageGraph {
        roots: vec![root.name],
        packages,
        config_version: 1,
    })
}

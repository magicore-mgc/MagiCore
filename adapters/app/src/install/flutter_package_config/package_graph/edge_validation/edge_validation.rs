//! Reject root or package edges absent from the assembled package map.
//! Từ chối edge root hoặc package không có trong map graph hoàn chỉnh.

use std::collections::{BTreeMap, BTreeSet};

use super::super::super::types::FlutterPackageGraphEntry;
use super::super::FlutterPackageGraphError;

pub(in crate::install::flutter_package_config::package_graph) fn validate_package_edges(
    entries: &BTreeMap<String, FlutterPackageGraphEntry>,
    root_dependencies: &[String],
    root_dev_dependencies: &[String],
) -> Result<(), FlutterPackageGraphError> {
    let known_packages = entries.keys().cloned().collect::<BTreeSet<_>>();
    for entry in entries.values() {
        for dependency in &entry.dependencies {
            if !known_packages.contains(dependency) {
                return Err(FlutterPackageGraphError::UnresolvedPackageEdge {
                    package: entry.name.clone(),
                    dependency: dependency.clone(),
                });
            }
        }
    }
    for dependency in root_dependencies.iter().chain(root_dev_dependencies) {
        if !known_packages.contains(dependency) {
            return Err(FlutterPackageGraphError::UnresolvedRootDependency(
                dependency.clone(),
            ));
        }
    }
    Ok(())
}

//! Wires the Dart resolved package graph owner.
//! Wiring owner graph package Dart đã resolve.

mod edge_validation;
mod error;
mod package_graph;
mod resolved_packages;
mod root_manifest;
mod sdk_packages;

use mgc_types::{MgError, MgResult, ResolvedGraph};
use std::path::Path;

use self::error::FlutterPackageGraphError;
use self::package_graph::build_flutter_package_graph_inner;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) fn build_flutter_package_graph(
    graph: &ResolvedGraph,
    project_root: &Path,
    sdk_root: Option<&Path>,
    sdk_packages: &[String],
) -> MgResult<super::types::FlutterPackageGraph> {
    build_flutter_package_graph_inner(graph, project_root, sdk_root, sdk_packages)
        .map_err(MgError::from)
}

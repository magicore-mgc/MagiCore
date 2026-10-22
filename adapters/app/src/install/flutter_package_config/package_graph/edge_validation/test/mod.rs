//! Wire full graph edge-validation tests.
//! Nối test kiểm tra edge toàn graph.

#![allow(clippy::unwrap_used)]

use super::super::super::types::FlutterPackageGraphEntry;
use super::super::FlutterPackageGraphError;
use super::*;
use std::collections::BTreeMap;

fn entries(rows: &[(&str, &[&str])]) -> BTreeMap<String, FlutterPackageGraphEntry> {
    rows.iter()
        .map(|(name, dependencies)| {
            let name = (*name).to_string();
            (
                name.clone(),
                FlutterPackageGraphEntry {
                    name,
                    version: "1.0.0".to_string(),
                    dependencies: dependencies.iter().map(|dep| (*dep).to_string()).collect(),
                    dev_dependencies: None,
                },
            )
        })
        .collect()
}

#[path = "missing_package_edge.rs"]
mod missing_package_edge;
#[path = "missing_root_edge.rs"]
mod missing_root_edge;
#[path = "valid_edges.rs"]
mod valid_edges;

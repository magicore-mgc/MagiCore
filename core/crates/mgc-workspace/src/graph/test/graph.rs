//! Workspace graph value-operation tests.
//! Test thao tác trên giá trị graph workspace.

#![allow(clippy::unwrap_used)]

use crate::graph::{WorkspaceEdge, WorkspaceGraph, WorkspaceNode, WorkspacePackageManifest};
use std::path::PathBuf;

#[test]
fn graph_reports_edges_and_node_count() {
    let manifest = WorkspacePackageManifest::default();
    let graph = WorkspaceGraph {
        nodes: vec![
            WorkspaceNode {
                name: "app".to_string(),
                path: PathBuf::from("app"),
                manifest: manifest.clone(),
            },
            WorkspaceNode {
                name: "shared".to_string(),
                path: PathBuf::from("shared"),
                manifest,
            },
        ],
        edges: vec![WorkspaceEdge { from: 0, to: 1 }],
    };

    assert_eq!(graph.edges_from(0), vec![1]);
    assert_eq!(graph.len(), 2);
    assert!(!graph.is_empty());
}

#[test]
fn empty_graph_reports_no_nodes_or_edges() {
    let graph = WorkspaceGraph::default();

    assert!(graph.edges_from(0).is_empty());
    assert_eq!(graph.len(), 0);
    assert!(graph.is_empty());
}

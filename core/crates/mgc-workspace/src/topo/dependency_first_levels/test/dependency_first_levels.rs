//! Dependency execution order contract tests.
//! Test contract thứ tự thực thi dependency.
#![allow(clippy::unwrap_used)]

use crate::{
    TopoError, WorkspaceEdge, WorkspaceGraph, WorkspaceNode, WorkspacePackageManifest,
    dependency_first_levels,
};
use std::path::PathBuf;

fn node(name: &str) -> WorkspaceNode {
    WorkspaceNode {
        name: name.to_string(),
        path: PathBuf::from(name),
        manifest: WorkspacePackageManifest {
            name: name.to_string(),
            ..Default::default()
        },
    }
}

#[test]
fn schedules_dependency_level_before_dependent_level() {
    let graph = WorkspaceGraph {
        nodes: vec![node("app"), node("library")],
        edges: vec![WorkspaceEdge { from: 0, to: 1 }],
    };

    assert_eq!(
        dependency_first_levels(&graph).unwrap(),
        vec![vec![1], vec![0]]
    );
}

#[test]
fn keeps_independent_packages_in_one_level() {
    let graph = WorkspaceGraph {
        nodes: vec![node("app"), node("library"), node("tools")],
        edges: Vec::new(),
    };

    assert_eq!(
        dependency_first_levels(&graph).unwrap(),
        vec![vec![0, 1, 2]]
    );
}

#[test]
fn returns_no_levels_for_an_empty_graph() {
    let graph = WorkspaceGraph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    assert!(dependency_first_levels(&graph).unwrap().is_empty());
}

#[test]
fn preserves_cycle_diagnostic_from_topological_ordering() {
    let graph = WorkspaceGraph {
        nodes: vec![node("a"), node("b")],
        edges: vec![
            WorkspaceEdge { from: 0, to: 1 },
            WorkspaceEdge { from: 1, to: 0 },
        ],
    };

    let error = dependency_first_levels(&graph).unwrap_err();

    assert!(matches!(error, TopoError::Cycle(_)));
}

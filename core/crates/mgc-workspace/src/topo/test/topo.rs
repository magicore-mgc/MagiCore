//! Kahn ordering and cycle behavior tests.
//! Test thứ tự Kahn và hành vi cycle.

#![allow(clippy::unwrap_used)]

use crate::{WorkspaceEdge, WorkspaceGraph, WorkspaceNode, WorkspacePackageManifest, topo_levels};
use std::path::PathBuf;

fn node(name: &str, rel: &str) -> WorkspaceNode {
    WorkspaceNode {
        name: name.to_string(),
        path: PathBuf::from(rel),
        manifest: WorkspacePackageManifest {
            name: name.to_string(),
            ..Default::default()
        },
    }
}

#[test]
fn topo_chain_orders_levels() {
    let graph = WorkspaceGraph {
        nodes: vec![node("a", "a"), node("b", "b"), node("c", "c")],
        edges: vec![
            WorkspaceEdge { from: 0, to: 1 },
            WorkspaceEdge { from: 1, to: 2 },
        ],
    };
    let levels = topo_levels(&graph).unwrap();
    assert_eq!(levels.len(), 3);
    assert_eq!(levels[0], vec![0]); // Node a is the first level — node a ở level đầu tiên.
    assert_eq!(levels[1], vec![1]); // Node b is the second level — node b ở level thứ hai.
    assert_eq!(levels[2], vec![2]); // Node c is the third level — node c ở level thứ ba.
}

#[test]
fn topo_fanout_same_level() {
    let graph = WorkspaceGraph {
        nodes: vec![node("a", "a"), node("b", "b"), node("c", "c")],
        edges: vec![
            WorkspaceEdge { from: 0, to: 1 },
            WorkspaceEdge { from: 0, to: 2 },
        ],
    };
    let levels = topo_levels(&graph).unwrap();
    assert_eq!(levels.len(), 2);
    let mut second = levels[1].clone();
    second.sort();
    assert_eq!(second, vec![1, 2]); // b and c share a level — b và c cùng level.
}

#[test]
fn topo_diamond() {
    let graph = WorkspaceGraph {
        nodes: vec![
            node("root", "root"),
            node("x", "x"),
            node("y", "y"),
            node("app", "app"),
        ],
        edges: vec![
            WorkspaceEdge { from: 0, to: 1 },
            WorkspaceEdge { from: 0, to: 2 },
            WorkspaceEdge { from: 1, to: 3 },
            WorkspaceEdge { from: 2, to: 3 },
        ],
    };
    let levels = topo_levels(&graph).unwrap();
    assert_eq!(levels.len(), 3);
    assert_eq!(levels[0], vec![0]);
    assert_eq!(levels[2], vec![3]);
}

#[test]
fn topo_cycle_detects() {
    let graph = WorkspaceGraph {
        nodes: vec![node("a", "a"), node("b", "b")],
        edges: vec![
            WorkspaceEdge { from: 0, to: 1 },
            WorkspaceEdge { from: 1, to: 0 },
        ],
    };
    let err = topo_levels(&graph).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("cycle"), "unexpected: {msg}");
}

#[test]
fn topo_empty() {
    let graph = WorkspaceGraph {
        nodes: vec![],
        edges: vec![],
    };
    let levels = topo_levels(&graph).unwrap();
    assert!(levels.is_empty());
}

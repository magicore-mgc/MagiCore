//! Workspace dependency graph contract tests.
//! Test hợp đồng cho graph dependency workspace.

#![allow(clippy::unwrap_used)]

use crate::{WorkspaceEdge, build_workspace_graph};

#[test]
fn build_graph_connects_all_workspace_dependency_categories() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let package_paths = ["app", "dep", "dev", "peer", "optional"];
    for package in package_paths {
        std::fs::create_dir_all(root.join(package)).unwrap();
    }
    std::fs::write(
        root.join("app/package.json"),
        r#"{
            "name":"app",
            "dependencies":{"dep":"workspace:*"},
            "devDependencies":{"dev":"workspace:*"},
            "peerDependencies":{"peer":"workspace:*"},
            "optionalDependencies":{"optional":"workspace:*"}
        }"#,
    )
    .unwrap();
    for package in ["dep", "dev", "peer", "optional"] {
        std::fs::write(
            root.join(package).join("package.json"),
            format!(r#"{{"name":"{package}"}}"#),
        )
        .unwrap();
    }

    let targets: Vec<_> = package_paths
        .iter()
        .map(|package| root.join(package))
        .collect();
    let graph = build_workspace_graph(&targets).unwrap();
    let mut edges = graph.edges.clone();
    edges.sort_by_key(|edge| (edge.from, edge.to));

    assert_eq!(graph.nodes.len(), 5);
    assert_eq!(
        edges,
        vec![
            WorkspaceEdge { from: 0, to: 1 },
            WorkspaceEdge { from: 0, to: 2 },
            WorkspaceEdge { from: 0, to: 3 },
            WorkspaceEdge { from: 0, to: 4 },
        ]
    );
}

#[test]
fn build_graph_rejects_an_unresolved_workspace_target() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("app");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("package.json"),
        r#"{"name":"app","dependencies":{"missing":"workspace:*"}}"#,
    )
    .unwrap();

    let error = build_workspace_graph(std::slice::from_ref(&root)).unwrap_err();

    assert!(error.to_string().contains("missing"));
    assert!(error.to_string().contains(&root.display().to_string()));
}

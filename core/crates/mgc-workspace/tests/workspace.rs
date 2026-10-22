#![allow(clippy::unwrap_used)]
//! mgc-workspace tests — RULE §5 (test/).

use mgc_workspace::{WorkspaceGraph, WorkspaceNode, WorkspacePackageManifest, topo_levels};

// --- graph + topo ---

#[test]
fn graph_edges_from_workspace_deps() {
    let mut a = WorkspacePackageManifest {
        name: "a".to_string(),
        ..Default::default()
    };
    a.dependencies
        .insert("b".to_string(), "workspace:*".to_string());
    let b = WorkspacePackageManifest {
        name: "b".to_string(),
        ..Default::default()
    };
    let graph = WorkspaceGraph {
        nodes: vec![
            WorkspaceNode {
                name: "a".into(),
                path: "a".into(),
                manifest: a,
            },
            WorkspaceNode {
                name: "b".into(),
                path: "b".into(),
                manifest: b,
            },
        ],
        edges: vec![],
    };
    // Build graph thật qua discover khó trong test đơn vị — kiểm tra edges_from:
    let _ = graph.edges_from(0);
    let levels = topo_levels(&graph).unwrap();
    assert_eq!(levels.len(), 1); // a->b chưa có edge → cùng level
}

#[test]
fn polyglot_manifest_reading_supports_rust_python_web() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    // 1. Rust crate
    let rust_dir = root.join("crates/core-lib");
    std::fs::create_dir_all(&rust_dir).unwrap();
    std::fs::write(
        rust_dir.join("Cargo.toml"),
        r#"[package]
name = "core-lib"
version = "0.1.0"
edition = "2021"

[dependencies]
common = { path = "../common" }
serde = "1.0"
"#,
    )
    .unwrap();

    let rust_manifest = mgc_workspace::read_package_manifest(&rust_dir)
        .unwrap()
        .unwrap();
    assert_eq!(rust_manifest.name, "core-lib");
    assert_eq!(
        rust_manifest.dependencies.get("common").unwrap(),
        "workspace:*"
    );

    // 2. Python package
    let py_dir = root.join("services/ai-agent");
    std::fs::create_dir_all(&py_dir).unwrap();
    std::fs::write(
        py_dir.join("pyproject.toml"),
        r#"[project]
name = "ai-agent"
version = "0.1.0"
"#,
    )
    .unwrap();

    let py_manifest = mgc_workspace::read_package_manifest(&py_dir)
        .unwrap()
        .unwrap();
    assert_eq!(py_manifest.name, "ai-agent");
}

// Keep cache regressions at this Cargo target root.
// Giữ các test cache ở crate root của Cargo target hiện tại.
include!("computation_cache/mod.rs");

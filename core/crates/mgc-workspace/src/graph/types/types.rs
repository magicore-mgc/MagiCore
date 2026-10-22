//! Shared workspace package, node, and edge data contracts.
//! Hợp đồng dữ liệu package, node và edge workspace dùng chung.

use std::collections::HashMap;
use std::path::PathBuf;

/// Package manifest subset used by workspace graph construction.
/// Phần manifest package được dùng để dựng graph workspace.
#[derive(Debug, Clone, Default)]
pub struct WorkspacePackageManifest {
    pub name: String,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
    pub peer_dependencies: HashMap<String, String>,
    pub optional_dependencies: HashMap<String, String>,
}

/// One workspace package in the graph.
/// Một package workspace trong graph.
#[derive(Debug, Clone)]
pub struct WorkspaceNode {
    pub name: String,
    pub path: PathBuf,
    pub manifest: WorkspacePackageManifest,
}

/// Directed workspace dependency edge, represented by node indices.
/// Cạnh dependency workspace có hướng, biểu diễn bằng chỉ số node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkspaceEdge {
    pub from: usize,
    pub to: usize,
}

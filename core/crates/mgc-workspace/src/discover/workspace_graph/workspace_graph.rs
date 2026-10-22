//! Workspace manifest dependency collection and graph construction.
//! Thu thập dependency manifest và dựng graph workspace.

use crate::{
    WorkspaceEdge, WorkspaceGraph, WorkspaceNode, WorkspacePackageManifest, read_package_manifest,
};
use std::path::PathBuf;

/// Build the workspace graph and connect dependencies using workspace: specifiers.
/// Dựng graph workspace và nối dependency bằng specifier workspace:.
pub fn build_workspace_graph(targets: &[PathBuf]) -> anyhow::Result<WorkspaceGraph> {
    let mut nodes = Vec::new();
    let mut name_to_index: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for path in targets {
        if let Some(manifest) = read_package_manifest(path)? {
            let index = nodes.len();
            name_to_index.insert(manifest.name.clone(), index);
            nodes.push(WorkspaceNode {
                name: manifest.name.clone(),
                path: path.clone(),
                manifest,
            });
        }
    }

    let mut edges = Vec::new();
    for (from_index, node) in nodes.iter().enumerate() {
        for (dep_name, spec) in manifest_deps(&node.manifest) {
            let trimmed = spec.trim();
            if !trimmed.starts_with("workspace:") {
                continue;
            }
            let Some(&to_index) = name_to_index.get(&dep_name) else {
                return Err(anyhow::anyhow!(
                    "workspace dependency '{}' referenced by '{}' was not found in workspace targets",
                    dep_name,
                    node.path.display()
                ));
            };
            edges.push(WorkspaceEdge {
                from: from_index,
                to: to_index,
            });
        }
    }

    Ok(WorkspaceGraph { nodes, edges })
}

/// Return dependencies from dependencies, dev, peer, and optional sections.
/// Trả dependency từ bốn nhóm dependencies, dev, peer và optional.
pub fn manifest_deps(manifest: &WorkspacePackageManifest) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for (name, spec) in &manifest.dependencies {
        out.push((name.clone(), spec.clone()));
    }
    for (name, spec) in &manifest.dev_dependencies {
        out.push((name.clone(), spec.clone()));
    }
    for (name, spec) in &manifest.peer_dependencies {
        out.push((name.clone(), spec.clone()));
    }
    for (name, spec) in &manifest.optional_dependencies {
        out.push((name.clone(), spec.clone()));
    }
    out
}

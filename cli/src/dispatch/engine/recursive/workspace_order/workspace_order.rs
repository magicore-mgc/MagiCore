//! Order workspace paths by the shared dependency-first graph contract.
//! Sắp path workspace theo contract graph dependency-first dùng chung.

use anyhow::Result;
use std::path::PathBuf;

/// Order graph-backed paths first and keep manifest-less paths in input order afterward.
/// Sắp path có graph trước; path thiếu manifest giữ input order ở cuối.
pub(in crate::dispatch::engine::recursive) fn order_workspaces(
    workspaces: Vec<PathBuf>,
) -> Result<Vec<PathBuf>> {
    let graph = mgc_workspace::build_workspace_graph(&workspaces)?;
    let levels = mgc_workspace::dependency_first_levels(&graph)?;
    let mut remaining = workspaces;
    let mut ordered = Vec::new();
    for level in levels {
        for index in level {
            let path = &graph.nodes[index].path;
            if let Some(position) = remaining.iter().position(|workspace| workspace == path) {
                ordered.push(remaining.remove(position));
            }
        }
    }
    ordered.append(&mut remaining);
    Ok(ordered)
}

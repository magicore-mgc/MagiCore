//! Workspace graph value operations.
//! Các thao tác trên giá trị graph workspace.

use super::types::{WorkspaceEdge, WorkspaceNode};

/// Workspace graph: nodes index by target order, edges = workspace:* dependencies.
/// Graph workspace: node theo thứ tự target, edge = dependency workspace:*.
#[derive(Debug, Clone, Default)]
pub struct WorkspaceGraph {
    pub nodes: Vec<WorkspaceNode>,
    pub edges: Vec<WorkspaceEdge>,
}

impl WorkspaceGraph {
    /// Edges leaving the node at `index`.
    /// Trả các cạnh đi ra từ node tại `index`.
    pub fn edges_from(&self, index: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter(|edge| edge.from == index)
            .map(|edge| edge.to)
            .collect()
    }

    /// Number of workspace nodes.
    /// Số node trong workspace.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Whether the graph has no nodes.
    /// Cho biết graph không có node.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

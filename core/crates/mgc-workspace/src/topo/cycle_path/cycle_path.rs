//! Builds the existing diagnostic path from remaining graph nodes.
//! Tạo đường dẫn chẩn đoán hiện có từ các node còn lại trong graph.

use crate::WorkspaceGraph;

/// Trace one cycle among nodes with nonzero remaining indegree.
/// Lần theo một cycle trong các node còn indegree khác không.
pub(in crate::topo) fn cycle_path(graph: &WorkspaceGraph, indegree: &[usize]) -> Vec<String> {
    let remaining: Vec<usize> = indegree
        .iter()
        .enumerate()
        .filter(|(_, degree)| **degree > 0)
        .map(|(index, _)| index)
        .collect();
    if remaining.is_empty() {
        return Vec::new();
    }
    // Follow edges until a node repeats — lần theo cạnh đến khi gặp lại node.
    let start = remaining[0];
    let mut path = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut current = start;
    loop {
        if !seen.insert(current) {
            break;
        }
        path.push(current);
        let next = graph
            .edges
            .iter()
            .find(|edge| edge.from == current)
            .map(|edge| edge.to)
            .or_else(|| remaining.iter().copied().find(|&r| r != current));
        let Some(next) = next else { break };
        current = next;
    }
    path.iter()
        .map(|&index| format!("'{}'", graph.nodes[index].name))
        .collect()
}

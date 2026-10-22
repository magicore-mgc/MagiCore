//! Kahn level computation for workspace dependency graphs.
//! Tính các level Kahn cho graph phụ thuộc workspace.

use super::{cycle_path::cycle_path, error::TopoError};
use crate::WorkspaceGraph;

/// Return dependency levels; nodes in one level can run in parallel.
/// Trả các level phụ thuộc; node cùng level có thể chạy song song.
pub fn topo_levels(graph: &WorkspaceGraph) -> Result<Vec<Vec<usize>>, TopoError> {
    let n = graph.nodes.len();
    let mut indegree = vec![0usize; n];
    let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); n];

    for edge in &graph.edges {
        indegree[edge.to] += 1;
        dependents[edge.from].push(edge.to);
    }

    let mut queue: std::collections::VecDeque<usize> = indegree
        .iter()
        .enumerate()
        .filter(|(_, degree)| **degree == 0)
        .map(|(index, _)| index)
        .collect();

    let mut levels: Vec<Vec<usize>> = Vec::new();
    let mut processed = 0usize;

    while !queue.is_empty() {
        let mut frontier = std::collections::VecDeque::new();
        let mut level = Vec::new();
        while let Some(node) = queue.pop_front() {
            processed += 1;
            level.push(node);
            for &next in &dependents[node] {
                indegree[next] -= 1;
                if indegree[next] == 0 {
                    frontier.push_back(next);
                }
            }
        }
        levels.push(level);
        queue = frontier;
    }

    if processed != n {
        let cycle = cycle_path(graph, &indegree);
        return Err(TopoError::Cycle(cycle));
    }

    Ok(levels)
}

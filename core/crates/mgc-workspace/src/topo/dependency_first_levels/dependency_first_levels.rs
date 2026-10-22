//! Reverse topological levels for graphs whose edges point from dependents to dependencies.
//! Đảo level topo khi cạnh graph đi từ package phụ thuộc tới dependency.

use super::super::{TopoError, topo_levels};
use crate::WorkspaceGraph;

/// Return dependency levels before the packages that depend on them.
/// Trả level dependency trước package phụ thuộc vào chúng.
pub fn dependency_first_levels(graph: &WorkspaceGraph) -> Result<Vec<Vec<usize>>, TopoError> {
    let mut levels = topo_levels(graph)?;
    levels.reverse();
    Ok(levels)
}

//! Wires the private dependency_graph owner modules.
//! Nối các module private thuộc owner dependency_graph.

mod dependency_graph;
pub(super) use dependency_graph::resolve_dependencies;

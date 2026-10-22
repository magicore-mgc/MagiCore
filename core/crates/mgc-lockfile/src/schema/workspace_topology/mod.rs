//! Wires cross-core workspace topology schema and tests.
//! Nối schema topology workspace liên core và test.

mod workspace_topology;
pub use self::workspace_topology::{CrossEdge, WorkspaceTopology};
#[cfg(test)]
mod test;

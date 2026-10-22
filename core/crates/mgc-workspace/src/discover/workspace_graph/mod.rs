//! Workspace graph-construction owner facade.
//! Facade owner dựng graph workspace.

mod workspace_graph;

pub use workspace_graph::build_workspace_graph;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

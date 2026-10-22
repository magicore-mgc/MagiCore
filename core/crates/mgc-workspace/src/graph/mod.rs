//! Workspace graph public module facade.
//! Facade module public cho graph workspace.

mod graph;
mod manifest_reader;
mod types;

pub use graph::WorkspaceGraph;
pub use manifest_reader::read_package_manifest;
pub use types::{WorkspaceEdge, WorkspaceNode, WorkspacePackageManifest};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

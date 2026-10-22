//! Workspace discovery module facade.
//! Facade của module tìm workspace.

mod discover;
mod layout_path;
mod project_scan;
mod workspace_config;
mod workspace_graph;

pub use discover::discover_workspace_targets;
pub use layout_path::{resolve_workspace_layout_dir, workspace_entry_is_directory};
pub use workspace_config::{DiscoverOptions, read_workspace_config};
pub use workspace_graph::build_workspace_graph;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

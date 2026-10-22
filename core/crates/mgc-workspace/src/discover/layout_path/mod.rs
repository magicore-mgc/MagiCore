//! Workspace path-safety owner facade.
//! Facade owner an toàn đường dẫn workspace.

mod layout_path;

pub use layout_path::{resolve_workspace_layout_dir, workspace_entry_is_directory};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

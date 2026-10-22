//! Workspace-configuration owner facade.
//! Facade của owner cấu hình workspace.

mod types;
mod workspace_config;

pub use types::DiscoverOptions;
pub use workspace_config::{read_workspace_config, workspace_layout};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

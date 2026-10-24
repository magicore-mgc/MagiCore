//! Local project discovery owner.
//! Owner dò project local.

mod local_projects;
mod reuse;

pub use local_projects::{LocalProject, copy_local_project, discover_local_projects};
pub use reuse::{find_reusable, resolve_offline_source, reuse_local_project};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

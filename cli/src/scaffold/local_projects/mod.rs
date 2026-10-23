//! Local project discovery owner.
//! Owner dò project local.

mod local_projects;

pub use local_projects::{LocalProject, copy_local_project, discover_local_projects};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

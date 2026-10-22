//! Workspace project-scan owner facade.
//! Facade owner quét project workspace.

mod project_scan;

pub(super) use project_scan::collect_projects;
#[cfg(test)]
pub use project_scan::has_backend_manifest;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

//! Source snapshot owner wiring — nối owner chụp trạng thái nguồn.

mod source_snapshot;

pub(super) use source_snapshot::{ProjectSources, inspect_project_sources};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

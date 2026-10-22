//! Workspace catalog loading facade.
//! Facade nạp catalog workspace.

mod load;

pub use load::load_workspace_catalogs;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

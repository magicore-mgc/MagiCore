//! Project scaffold owner — wiring only.
//! Owner scaffold project — chỉ wiring.

#[path = "project.rs"]
mod project;

pub use project::scaffold_project;

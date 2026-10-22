//! Project binary facade — export project execution entry point.
//! Facade binary project — export entry point chạy binary project.

#[path = "project_binary.rs"]
mod project_binary;
pub use project_binary::{
    run_project_binary, run_project_binary_inherited, run_project_binary_inherited_cancellable,
};

//! Optimizer project-directory and no-follow file boundary.
//! (Ranh giới thư mục optimizer và đọc file không theo symlink.)

mod project_files;

pub(super) use project_files::{read_optimizer_text, validate_optimizer_directory};

#[cfg(test)]
#[path = "test/project_files.rs"]
mod tests;

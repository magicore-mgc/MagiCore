//! Journal filesystem module wiring.
//! Wiring filesystem cho journal mutation.

#[path = "filesystem.rs"]
mod implementation;

pub(crate) use implementation::{atomic_write_file, fsync_dir, read_project_string};
pub(crate) use implementation::{atomic_write_file_checked, refuse_project_link};

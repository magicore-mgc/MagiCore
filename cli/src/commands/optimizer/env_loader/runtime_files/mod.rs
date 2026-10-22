//! Runtime-to-optimizer-file selection.
//! (Chọn file optimizer dựa trên runtime.)

mod runtime_files;

pub(super) use runtime_files::get_env_files_for_runtime;

#[cfg(test)]
#[path = "test/runtime_files.rs"]
mod tests;

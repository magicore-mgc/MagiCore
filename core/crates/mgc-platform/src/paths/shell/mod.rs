//! Install the managed executable directory into the current user's shell search path.
//! Cài thư mục executable do MGC quản lý vào PATH của user hiện tại.

mod shell;
#[cfg(not(windows))]
mod unix;
#[cfg(windows)]
mod windows;

pub use shell::{ensure_bin_on_path, ensure_global_bin_on_path};

//! Status-message wiring — wiring cho thông báo trạng thái.
//! Keeps the crate-root functions and their output channels unchanged.
//! Giữ nguyên các hàm root và kênh output.

mod status;
#[cfg(test)]
#[path = "test/status.rs"]
mod tests;

pub use status::{blank_line, error, hint, info, success, warning, warning_stderr};

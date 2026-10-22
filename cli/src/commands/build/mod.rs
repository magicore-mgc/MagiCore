//! Build command facade — export build command entry points.
//! Facade lệnh build — export entry point các lệnh build.

#[path = "build.rs"]
mod implementation;
pub use implementation::*;
pub(crate) use implementation::{node_bin_args, python_cmd, tool_unavailable};

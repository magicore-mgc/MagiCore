//! Windows cmd argv parser exports.
//! Export parser argv Windows cmd.

#[path = "windows_cmd.rs"]
mod implementation;
pub(in crate::run::implementation) use implementation::windows_cmd_invocation;

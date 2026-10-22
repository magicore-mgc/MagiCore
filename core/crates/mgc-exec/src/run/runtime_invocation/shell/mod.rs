//! POSIX-like shell argv parser exports.
//! Export parser argv shell kiểu POSIX.

#[path = "shell.rs"]
mod implementation;
pub(super) use implementation::shell_script_argument;

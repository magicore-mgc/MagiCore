//! Command-builder owner facade — export platform command setup.
//! Facade owner command builder — export cấu hình lệnh theo nền tảng.

#[path = "command_builder.rs"]
mod implementation;

pub(super) use implementation::build_command;

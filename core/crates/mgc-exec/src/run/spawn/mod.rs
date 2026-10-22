//! Spawn facade — export the shared child-process launcher.
//! Facade spawn — export bộ khởi chạy process con dùng chung.

#[path = "spawn.rs"]
mod implementation;
pub(super) use implementation::execute_command;

#[path = "windows_command/mod.rs"]
pub(in crate::run::implementation) mod windows_command;

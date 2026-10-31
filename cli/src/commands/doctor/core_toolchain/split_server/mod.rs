#[path = "split_server.rs"]
mod implementation;

pub(in crate::commands::doctor::core_toolchain) use implementation::split_server_half_tools;

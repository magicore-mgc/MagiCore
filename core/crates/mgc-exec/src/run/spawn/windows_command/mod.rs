//! Windows command preparation facade.
//! Facade chuẩn bị lệnh Windows.

#[cfg(any(windows, test))]
#[path = "error.rs"]
mod error;
#[path = "windows_command.rs"]
mod implementation;

#[cfg(windows)]
pub(in crate::run::implementation) use implementation::windows_batch_command;
#[cfg(any(windows, test))]
pub(in crate::run::implementation) use implementation::{
    find_windows_command, validate_windows_batch_invocation, windows_tool_path_units,
};
#[cfg(windows)]
pub(in crate::run::implementation) use implementation::{
    resolve_windows_shim, windows_system_tool_path,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

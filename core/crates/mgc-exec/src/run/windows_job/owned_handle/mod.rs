//! Windows native handle ownership and query facade.
//! Facade quản lý sở hữu handle native Windows và truy vấn thông tin.

#[path = "owned_handle.rs"]
mod implementation;

#[cfg(windows)]
pub(in crate::run::implementation::windows_job) use implementation::WindowsOwnedHandle;

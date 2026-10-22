//! Deferred native handle-close failure owner facade.
//! Facade owner lưu lỗi đóng native handle để báo lại sau.

#[path = "deferred_handle_close.rs"]
mod implementation;

#[cfg(windows)]
pub(in crate::run::implementation) use implementation::take_deferred_handle_close_error;

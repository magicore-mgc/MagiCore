//! Windows Job root-exit facade.
//! Facade xử lý khi root process Windows Job thoát.

#[path = "error.rs"]
mod error;
#[path = "root_exit.rs"]
mod implementation;

pub(super) use implementation::finish_monitored_root_exit;

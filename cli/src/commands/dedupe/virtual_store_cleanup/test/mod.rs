//! Virtual-store cleanup test wiring — registers deletion/accounting coverage.
//! Wiring test dọn virtual store — đăng ký kiểm thử xóa và tính byte.
#[cfg(any(unix, windows))]
mod root_safety;
#[cfg(any(unix, windows))]
mod symlink_accounting;
#[cfg(any(unix, windows))]
mod symlink_support;
mod virtual_store_cleanup;

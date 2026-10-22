//! Tests documented contention and non-contention error classes.
//! Kiểm tra loại lỗi contention và lỗi không phải contention.

use super::super::is_lock_contention;

#[test]
fn would_block_is_contention_but_unrelated_errors_are_not() {
    assert!(is_lock_contention(&std::io::Error::from(
        std::io::ErrorKind::WouldBlock
    )));
    assert!(!is_lock_contention(&std::io::Error::from(
        std::io::ErrorKind::PermissionDenied
    )));
}

#[cfg(windows)]
#[test]
fn fs2_windows_lock_violation_is_contention() {
    let error = std::io::Error::from_raw_os_error(
        windows_sys::Win32::Foundation::ERROR_LOCK_VIOLATION as i32,
    );
    assert!(is_lock_contention(&error));
}

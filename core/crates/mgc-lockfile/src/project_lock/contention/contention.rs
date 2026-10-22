//! Classifies only documented OS lock-conflict errors as contention.
//! Chỉ phân loại lỗi conflict lock OS đã biết là contention.

/// Treat WouldBlock and Windows ERROR_LOCK_VIOLATION as contention only.
/// Chỉ coi WouldBlock và Windows ERROR_LOCK_VIOLATION là contention.
pub(in crate::project_lock) fn is_lock_contention(error: &std::io::Error) -> bool {
    if error.kind() == std::io::ErrorKind::WouldBlock {
        return true;
    }
    #[cfg(windows)]
    {
        return error.raw_os_error()
            == Some(windows_sys::Win32::Foundation::ERROR_LOCK_VIOLATION as i32);
    }
    #[cfg(not(windows))]
    {
        false
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

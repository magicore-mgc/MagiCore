//! macOS physical memory API.
//! (API bộ nhớ vật lý của macOS.)

/// Read physical RAM through macOS sysctl directly, without spawning `sysctl`.
/// (Đọc RAM qua API sysctl của kernel macOS, không chạy executable `sysctl`.)
#[cfg(target_os = "macos")]
#[allow(unsafe_code)]
pub(in crate::commands::optimizer::detect::memory) fn macos_total_memory_bytes() -> Option<u64> {
    let mut value = 0_u64;
    let mut value_size = std::mem::size_of::<u64>();
    // SAFETY: the NUL-terminated MIB name and writable u64 buffer remain
    // valid for the duration of the syscall; `value_size` describes it.
    // (An toàn: tên MIB kết thúc NUL, buffer u64 ghi được còn sống suốt syscall.)
    let result = unsafe {
        libc::sysctlbyname(
            c"hw.memsize".as_ptr(),
            (&mut value as *mut u64).cast(),
            &mut value_size,
            std::ptr::null_mut(),
            0,
        )
    };
    (result == 0 && value_size == std::mem::size_of::<u64>() && value > 0).then_some(value)
}

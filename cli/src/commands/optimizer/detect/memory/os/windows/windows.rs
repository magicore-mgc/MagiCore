//! Windows physical memory API.
//! (API bộ nhớ vật lý của Windows.)

/// Read physical RAM through the Windows memory status API in-process.
/// (Đọc RAM vật lý qua Windows API ngay trong process.)
#[cfg(target_os = "windows")]
#[allow(unsafe_code)]
pub(super) fn windows_total_memory_bytes() -> Option<u64> {
    use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

    let mut status = MEMORYSTATUSEX {
        dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
        dwMemoryLoad: 0,
        ullTotalPhys: 0,
        ullAvailPhys: 0,
        ullTotalPageFile: 0,
        ullAvailPageFile: 0,
        ullTotalVirtual: 0,
        ullAvailVirtual: 0,
        ullAvailExtendedVirtual: 0,
    };
    // SAFETY: `status` is initialized with the exact structure size and
    // remains writable for the duration of the Windows API call.
    // (An toàn: `status` ghi được, khai báo đúng kích thước suốt lời gọi API.)
    let succeeded = unsafe { GlobalMemoryStatusEx(&mut status) };
    (succeeded != 0 && status.ullTotalPhys > 0).then_some(status.ullTotalPhys)
}

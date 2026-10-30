// Tests limited-query metadata access through the RAII handle owner.
// Kiểm tra truy vấn metadata giới hạn qua RAII handle owner.

use super::WindowsOwnedHandle;

#[test]
#[allow(unsafe_code)]
fn owned_handle_reads_current_process_identity() {
    use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};

    let process_id = std::process::id();
    // SAFETY: test-only operation on process-local state or FFI with
    // caller-owned arguments valid for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process hoặc FFI với
    // đối số do caller sở hữu, hợp lệ trong lời gọi.
    let raw = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, process_id) };
    assert!(!raw.is_null(), "current process query handle must open");
    let handle = WindowsOwnedHandle::from_raw(raw);
    assert!(!handle.full_image_path().unwrap().as_os_str().is_empty());
    assert!(!handle.has_terminated().unwrap());
}

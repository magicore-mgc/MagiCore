//! Open, verify and retain a Job member handle across metadata inspection.
//! Mở, xác minh và giữ handle Job member trong suốt quá trình đọc metadata.

#[cfg(windows)]
use super::super::super::error::{WindowsApiFailure, WindowsJobError};
#[cfg(windows)]
use super::super::owned_handle::WindowsOwnedHandle;

#[cfg(windows)]
#[allow(unsafe_code)]
pub(in crate::run::implementation::windows_job::implementation) fn capture_job_member_process(
    job: windows_sys::Win32::Foundation::HANDLE,
    process_id: u32,
) -> std::result::Result<Option<WindowsOwnedHandle>, WindowsJobError> {
    use windows_sys::Win32::Foundation::ERROR_INVALID_PARAMETER;
    use windows_sys::Win32::System::JobObjects::IsProcessInJob;
    use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};

    // SAFETY: this opens only the PID returned by our private Job Object query and requests
    // query-only access; an invalid PID means that process exited during the snapshot.
    // AN TOÀN: chỉ mở PID do Job Object riêng trả về với quyền query; PID sai nghĩa là process
    // đã thoát trong lúc chụp trạng thái.
    let process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, process_id) };
    if process.is_null() {
        let error = super::super::windows_last_error();
        if error == ERROR_INVALID_PARAMETER {
            if !super::super::process_snapshot::active_process_ids(job)?.contains(&process_id) {
                // A fresh Job query confirms this PID is no longer an active member.
                // Truy vấn Job mới xác nhận PID này không còn là thành viên đang chạy.
                return Ok(None);
            }
            return Err(WindowsJobError::ActiveMemberUnavailable {
                process_id,
                source: WindowsApiFailure::new("OpenProcess", error),
            });
        }
        return Err(WindowsJobError::native_api("OpenProcess", error));
    }
    let process = WindowsOwnedHandle::from_raw(process);
    let mut is_member = 0;
    // SAFETY: both handles are live and `is_member` points to writable BOOL storage.
    // AN TOÀN: cả hai handle còn sống và `is_member` trỏ tới vùng BOOL có thể ghi.
    let queried = unsafe { IsProcessInJob(process.as_raw(), job, &mut is_member) };
    if queried == 0 {
        return Err(WindowsJobError::MembershipQueryFailed {
            process_id,
            source: WindowsApiFailure::new("IsProcessInJob", super::super::windows_last_error()),
        });
    }
    if is_member == 0 {
        return Err(WindowsJobError::MembershipChanged { process_id });
    }
    // Keep this handle alive through metadata capture so Windows cannot reuse its PID.
    // Giữ handle này tới khi chụp metadata để Windows không tái sử dụng PID.
    Ok(Some(process))
}

#[cfg(all(test, windows))]
#[path = "test/mod.rs"]
mod test;

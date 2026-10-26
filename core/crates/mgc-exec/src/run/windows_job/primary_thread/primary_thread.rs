//! Locate exactly one thread owned by the suspended child and resume it once.
//! Tìm thread thuộc child đang suspend và chỉ resume thread đó một lần.

#[cfg(windows)]
use super::super::super::error::WindowsJobError;

// SAFETY: the toolhelp snapshot and THREADENTRY32 struct are created locally,
// initialised to zero, and every Win32 walk call is checked before the next
// entry is read; handles obtained from the snapshot are closed in this scope.
// AN TOÀN: snapshot toolhelp và struct THREADENTRY32 tạo cục bộ, khởi tạo 0,
// mọi lời gọi walk đều được kiểm trước khi đọc entry kế; handle lấy từ
// snapshot được đóng trong scope này.
#[cfg(windows)]
#[allow(unsafe_code)]
pub(in crate::run::implementation::windows_job::implementation) fn resume_primary_thread(
    process_id: u32,
) -> std::result::Result<(), WindowsJobError> {
    use windows_sys::Win32::Foundation::{ERROR_NO_MORE_FILES, SetLastError};
    use windows_sys::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, TH32CS_SNAPTHREAD, THREADENTRY32, Thread32First, Thread32Next,
    };
    use windows_sys::Win32::System::Threading::{OpenThread, ResumeThread, THREAD_SUSPEND_RESUME};

    // SAFETY: a thread snapshot takes no pointers and no borrowed data; the
    // returned handle is checked against INVALID_HANDLE_VALUE and owned by
    // `WindowsOwnedHandle`, so it is closed exactly once.
    // AN TOÀN: snapshot thread không nhận con trỏ hay dữ liệu mượn; handle trả về
    // được kiểm với INVALID_HANDLE_VALUE và thuộc `WindowsOwnedHandle`, nên đóng
    // đúng một lần.
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
    if snapshot == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
        return Err(WindowsJobError::native_api(
            "CreateToolhelp32Snapshot",
            super::super::windows_last_error(),
        ));
    }
    let snapshot = super::super::owned_handle::WindowsOwnedHandle::from_raw(snapshot);
    let mut entry = THREADENTRY32 {
        dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
        ..Default::default()
    };
    let mut found_thread = None;
    // SAFETY: clearing the thread-local last-error has no preconditions.
    // AN TOÀN: xoá last-error của thread hiện tại không có điều kiện tiên quyết.
    unsafe { SetLastError(0) };
    // SAFETY: `snapshot` is the live handle created above and `entry.dwSize` is
    // the size of the struct being written, which the API requires.
    // AN TOÀN: `snapshot` là handle còn sống tạo ở trên và `entry.dwSize` đúng
    // bằng kích thước struct được ghi, đúng yêu cầu API.
    let first_entry_found = unsafe { Thread32First(snapshot.as_raw(), &mut entry) } != 0;
    if !first_entry_found {
        let error = super::super::windows_last_error();
        if error != ERROR_NO_MORE_FILES {
            return Err(WindowsJobError::native_api("Thread32First", error));
        }
    }
    let mut has_entry = first_entry_found;
    while has_entry {
        if entry.th32OwnerProcessID == process_id {
            found_thread = Some(entry.th32ThreadID);
            break;
        }
        // SAFETY: same as above — clearing last-error has no preconditions.
        // AN TOÀN: như trên — xoá last-error không có điều kiện tiên quyết.
        unsafe { SetLastError(0) };
        // SAFETY: `snapshot` is live and `entry` keeps its `dwSize` from the
        // first successful call, which is what continues the enumeration.
        // AN TOÀN: `snapshot` còn sống và `entry` giữ `dwSize` từ lời gọi thành
        // công đầu tiên, đúng điều kiện để tiếp tục liệt kê.
        has_entry = unsafe { Thread32Next(snapshot.as_raw(), &mut entry) } != 0;
        if !has_entry {
            let error = super::super::windows_last_error();
            if error != ERROR_NO_MORE_FILES {
                return Err(WindowsJobError::native_api("Thread32Next", error));
            }
        }
    }
    let thread_id = found_thread.ok_or(WindowsJobError::MissingPrimaryThread { process_id })?;
    // SAFETY: opening an existing thread by id with only the resume access bit;
    // no borrowed data is passed and the handle is checked for null.
    // AN TOÀN: mở thread đã tồn tại theo id với duy nhất quyền resume; không
    // truyền dữ liệu mượn và handle được kiểm null.
    let thread = unsafe { OpenThread(THREAD_SUSPEND_RESUME, 0, thread_id) };
    if thread.is_null() {
        return Err(WindowsJobError::native_api(
            "OpenThread",
            super::super::windows_last_error(),
        ));
    }
    let thread = super::super::owned_handle::WindowsOwnedHandle::from_raw(thread);
    // SAFETY: `thread` is a live handle owned above with THREAD_SUSPEND_RESUME
    // access, which is exactly what ResumeThread requires.
    // AN TOÀN: `thread` là handle còn sống, sở hữu ở trên, có quyền
    // THREAD_SUSPEND_RESUME — đúng thứ ResumeThread yêu cầu.
    let previous_suspend_count = unsafe { ResumeThread(thread.as_raw()) };
    if previous_suspend_count == u32::MAX {
        return Err(WindowsJobError::native_api(
            "ResumeThread",
            super::super::windows_last_error(),
        ));
    }
    validate_primary_suspend_count(process_id, previous_suspend_count)
}

#[cfg(windows)]
fn validate_primary_suspend_count(
    process_id: u32,
    previous_suspend_count: u32,
) -> std::result::Result<(), WindowsJobError> {
    if previous_suspend_count == 1 {
        return Ok(());
    }
    Err(WindowsJobError::UnexpectedSuspendCount {
        process_id,
        count: previous_suspend_count,
    })
}

#[cfg(all(test, windows))]
#[path = "test/mod.rs"]
mod test;

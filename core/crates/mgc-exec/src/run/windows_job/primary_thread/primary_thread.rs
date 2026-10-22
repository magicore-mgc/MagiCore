//! Locate exactly one thread owned by the suspended child and resume it once.
//! Tìm thread thuộc child đang suspend và chỉ resume thread đó một lần.

#[cfg(windows)]
use super::super::super::error::WindowsJobError;

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
    unsafe { SetLastError(0) };
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
        unsafe { SetLastError(0) };
        has_entry = unsafe { Thread32Next(snapshot.as_raw(), &mut entry) } != 0;
        if !has_entry {
            let error = super::super::windows_last_error();
            if error != ERROR_NO_MORE_FILES {
                return Err(WindowsJobError::native_api("Thread32Next", error));
            }
        }
    }
    let thread_id = found_thread.ok_or(WindowsJobError::MissingPrimaryThread { process_id })?;
    let thread = unsafe { OpenThread(THREAD_SUSPEND_RESUME, 0, thread_id) };
    if thread.is_null() {
        return Err(WindowsJobError::native_api(
            "OpenThread",
            super::super::windows_last_error(),
        ));
    }
    let thread = super::super::owned_handle::WindowsOwnedHandle::from_raw(thread);
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

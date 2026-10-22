//! Per-platform configuration for child-process tree isolation.
//! Cấu hình cô lập cây process con theo từng nền tảng.

use super::super::ProcessTreeGuard;
#[path = "error.rs"]
mod error;
use anyhow::Result;
use error::ProcessInspectionUnavailable;
use std::process::Command;

/// Report whether the current platform provides a process-tree isolation guard.
/// Báo cáo nền tảng hiện tại có guard cô lập cây process hay không.
pub fn process_tree_guard_available() -> bool {
    // Unix session isolation and Windows Job Objects are always available
    // on the respective supported targets; other targets opt out.
    // (Cô lập session Unix và Job Object Windows luôn có trên target hỗ trợ;
    // target khác không tham gia.)
    cfg!(any(unix, windows))
}

pub(in crate::run::implementation) fn ensure_process_inspection_available(
    monitor_enabled: bool,
    inspector_available: bool,
) -> std::result::Result<(), ProcessInspectionUnavailable> {
    if monitor_enabled && !inspector_available {
        return Err(process_inspection_unavailable_error());
    }
    Ok(())
}

pub(in crate::run::implementation) fn process_inspection_unavailable_error()
-> ProcessInspectionUnavailable {
    ProcessInspectionUnavailable
}

#[cfg(unix)]
pub(in crate::run::implementation) fn configure_process_isolation(
    command: &mut Command,
) -> Result<ProcessTreeGuard> {
    use std::os::unix::process::CommandExt;

    // A private session prevents descendants from joining caller-owned groups.
    // Session riêng ngăn descendant nhập process group thuộc caller.
    // SAFETY: the pre-exec closure only calls async-signal-safe `setsid` after fork.
    // AN TOÀN: closure pre-exec chỉ gọi `setsid` async-signal-safe sau fork.
    #[allow(unsafe_code)]
    unsafe {
        command.pre_exec(|| {
            // SAFETY: `setsid` changes only the child process's session before exec.
            // AN TOÀN: `setsid` chỉ đổi session của child trước khi exec.
            let session_id = libc::setsid();
            if session_id == -1 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        });
    }
    Ok(ProcessTreeGuard)
}

#[cfg(windows)]
pub(in crate::run::implementation) fn configure_process_isolation(
    command: &mut Command,
) -> Result<ProcessTreeGuard> {
    use std::os::windows::process::CommandExt;
    use windows_sys::Win32::System::Threading::CREATE_SUSPENDED;

    // Keep the primary thread paused until its process is assigned to the job.
    // Tiến trình con chưa chạy cho tới khi đã được gắn vào job quản lý cây.
    // Source: https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
    // Nguồn: tài liệu Microsoft mô tả CREATE_SUSPENDED giữ thread chính trước khi chạy.
    command.creation_flags(CREATE_SUSPENDED);
    ProcessTreeGuard::new()
}

#[cfg(not(any(unix, windows)))]
pub(in crate::run::implementation) fn configure_process_isolation(
    _command: &mut Command,
) -> Result<ProcessTreeGuard> {
    Ok(ProcessTreeGuard)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

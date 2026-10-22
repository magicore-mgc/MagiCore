//! Capture a refreshed Windows process table and query native creation identity.
//! Chụp process table Windows mới và truy vấn creation identity native.

#[cfg(all(windows, test))]
use super::super::{
    WindowsProcessAncestry, WindowsProcessScan, WindowsProcessSnapshot, retry_windows_process_scan,
    scan_windows_process_snapshot,
};
#[cfg(all(windows, test))]
use anyhow::{Result, bail};
#[cfg(all(windows, test))]
use std::path::Path;
#[cfg(all(windows, test))]
use std::time::{Duration, Instant};

#[cfg(all(windows, test))]
const WINDOWS_FILETIME_TICKS_PER_SECOND: u64 = 10_000_000;
#[cfg(all(windows, test))]
const WINDOWS_FILETIME_UNIX_EPOCH_OFFSET_SECONDS: u64 = 11_644_473_600;

#[cfg(all(windows, test))]
pub(super) fn find_forbidden_descendant_with_timeout(
    root_pid: u32,
    exempt: &[&str],
    _shadow_dir: &Path,
    started: Instant,
    timeout: Option<Duration>,
    ancestry: &mut WindowsProcessAncestry,
    child_has_exited: impl FnMut() -> Result<bool>,
) -> Result<WindowsProcessScan> {
    use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};
    let mut system = System::new();
    let root = Pid::from_u32(root_pid);
    retry_windows_process_scan(started, timeout, child_has_exited, || {
        // A full refresh discovers descendants created while an earlier command line was blank.
        // Làm mới toàn bộ giúp phát hiện con cháu được tạo khi command line lượt trước còn trống.
        system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing().with_cmd(UpdateKind::Always),
        );
        if system.process(root).is_none() {
            bail!("process table did not contain monitored root PID {root_pid}");
        }
        let snapshot = system
            .processes()
            .iter()
            .map(|(pid, process)| WindowsProcessSnapshot {
                pid: pid.as_u32(),
                parent_pid: process.parent().map(|parent| parent.as_u32()),
                creation_time: None,
                image_name: process.name().to_string_lossy().into_owned(),
                command: process.cmd().to_vec(),
            })
            .collect::<Vec<_>>();
        let observed_start_times = system
            .processes()
            .iter()
            .map(|(pid, process)| (pid.as_u32(), process.start_time()))
            .collect::<std::collections::HashMap<_, _>>();
        scan_windows_process_snapshot(root_pid, ancestry, &snapshot, exempt, |pid| {
            let Some(observed_start_time) = observed_start_times.get(&pid) else {
                return Ok(None);
            };
            let creation_time = windows_process_creation_time(pid);
            // Match the live handle to the refreshed row before accepting its exact creation time.
            // Đối chiếu handle đang sống với dòng vừa refresh trước khi nhận thời điểm tạo chính xác.
            Ok(creation_time.filter(|creation_time| {
                (*creation_time / WINDOWS_FILETIME_TICKS_PER_SECOND)
                    .checked_sub(WINDOWS_FILETIME_UNIX_EPOCH_OFFSET_SECONDS)
                    == Some(*observed_start_time)
            }))
        })
    })
}

#[cfg(all(windows, test))]
#[allow(unsafe_code)]
fn windows_process_creation_time(pid: u32) -> Option<u64> {
    use windows_sys::Win32::Foundation::{CloseHandle, FILETIME};
    use windows_sys::Win32::System::Threading::{
        GetProcessTimes, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
    };

    // SAFETY: this PID comes from the refreshed process table; the requested access is query-only.
    // (An toàn: PID lấy từ bảng process vừa refresh; quyền mở chỉ dùng để truy vấn thông tin.)
    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
    if handle.is_null() {
        return None;
    }

    let mut creation_time = FILETIME::default();
    let mut exit_time = FILETIME::default();
    let mut kernel_time = FILETIME::default();
    let mut user_time = FILETIME::default();
    // SAFETY: handle is valid from OpenProcess and all output pointers reference live FILETIME values.
    // (An toàn: handle còn hiệu lực từ OpenProcess; các con trỏ output trỏ tới FILETIME còn sống.)
    let query_succeeded = unsafe {
        GetProcessTimes(
            handle,
            &mut creation_time,
            &mut exit_time,
            &mut kernel_time,
            &mut user_time,
        ) != 0
    };
    // SAFETY: handle is the non-null handle returned by OpenProcess above and is closed exactly once.
    // (An toàn: đóng đúng một lần handle khác null do OpenProcess trả về ở trên.)
    let close_succeeded = unsafe { CloseHandle(handle) != 0 };
    if !query_succeeded || !close_succeeded {
        return None;
    }
    Some(((creation_time.dwHighDateTime as u64) << 32) | creation_time.dwLowDateTime as u64)
}

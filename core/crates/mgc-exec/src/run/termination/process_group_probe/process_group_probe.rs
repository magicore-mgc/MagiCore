//! Read process-group identity and observe live descendant groups.
//! Đọc identity process group và quan sát group hậu duệ còn sống.

#[cfg(unix)]
pub(in crate::run::implementation) fn process_group_is_live(group: u32) -> std::io::Result<bool> {
    let group = libc::pid_t::try_from(group).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "process-group ID does not fit the platform process identifier type",
        )
    })?;
    // Recheck that the numeric PGID still names a live group before signaling it.
    // Kiểm tra PGID bằng số vẫn còn là group sống ngay trước khi gửi signal.
    // SAFETY: signal zero only probes existence and permission; it changes no process state.
    // AN TOÀN: signal 0 chỉ dò tồn tại và quyền truy cập, không đổi trạng thái process.
    #[allow(unsafe_code)]
    let result = unsafe { libc::kill(-group, 0) };
    if result == 0 {
        return Ok(true);
    }
    let source = std::io::Error::last_os_error();
    match source.raw_os_error() {
        Some(libc::ESRCH) => Ok(false),
        // Permission denial still proves that a group exists; the subsequent signal attempt
        // reports its own typed error if this runner cannot terminate that group.
        // Permission denial vẫn cho biết group tồn tại; lần gửi signal sau sẽ trả lỗi typed
        // nếu runner không được phép dừng group đó.
        Some(libc::EPERM) => Ok(true),
        _ => Err(source),
    }
}

#[cfg(unix)]
pub(in crate::run::implementation) fn process_group_id(
    pid: u32,
) -> std::result::Result<Option<u32>, super::super::TerminationError> {
    let process_id = libc::pid_t::try_from(pid).map_err(|_| {
        super::super::TerminationError::process_group_identity(
            pid,
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "process ID does not fit the platform process identifier type",
            ),
        )
    })?;
    // `getpgid` reads membership; ESRCH is the expected race for an exited PID.
    // `getpgid` chỉ đọc membership; ESRCH là race bình thường khi PID vừa thoát.
    // SAFETY: the PID comes from an OS process snapshot and the call only reads its PGID.
    // AN TOÀN: PID lấy từ snapshot của OS và lời gọi chỉ đọc PGID.
    #[allow(unsafe_code)]
    let process_group = unsafe { libc::getpgid(process_id) };
    if process_group < 0 {
        let source = std::io::Error::last_os_error();
        if source.raw_os_error() == Some(libc::ESRCH) {
            return Ok(None);
        }
        return Err(super::super::TerminationError::process_group_identity(
            pid, source,
        ));
    }
    let process_group = u32::try_from(process_group).map_err(|_| {
        super::super::TerminationError::process_group_identity(
            pid,
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "operating system returned an invalid process-group ID",
            ),
        )
    })?;
    Ok(Some(process_group))
}

#[cfg(unix)]
pub(in crate::run::implementation) fn process_groups_have_descendants(
    system: &mut sysinfo::System,
    observed_process_groups: &std::collections::HashSet<u32>,
    root_pid: u32,
) -> std::result::Result<bool, super::super::TerminationError> {
    use sysinfo::{ProcessRefreshKind, ProcessesToUpdate};

    system.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing());
    for pid in system.processes().keys() {
        if pid.as_u32() == root_pid {
            continue;
        }
        let Some(group) = process_group_id(pid.as_u32())? else {
            continue;
        };
        if observed_process_groups.contains(&group) {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(all(unix, test))]
#[path = "test/mod.rs"]
mod tests;

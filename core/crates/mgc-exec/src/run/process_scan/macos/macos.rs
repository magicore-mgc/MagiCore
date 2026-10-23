//! macOS process snapshots — read BSD process identity and bounded process lists.
//! (Owner macOS cho start-time identity, process group và liệt kê PID có retry giới hạn.)

use super::super::types::ProcessScanSnapshot;
use anyhow::{Context, Result};
use std::collections::HashMap;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

#[cfg(target_os = "macos")]
const PROCESS_SNAPSHOT_CAPACITY_SLACK: usize = 256;
#[cfg(target_os = "macos")]
const PROCESS_SNAPSHOT_RETRIES: usize = 3;

#[cfg(target_os = "macos")]
pub(in crate::run::implementation) fn macos_process_snapshot(
    pid: u32,
) -> Option<ProcessScanSnapshot> {
    let pid = libc::pid_t::try_from(pid).ok()?;
    let expected_size = libc::c_int::try_from(std::mem::size_of::<libc::proc_bsdinfo>()).ok()?;
    let mut info = std::mem::MaybeUninit::<libc::proc_bsdinfo>::zeroed();
    // Read only BSD process identity fields; sysinfo also queries arguments and thread state.
    // Chỉ đọc trường định danh BSD; sysinfo còn truy vấn command line và trạng thái thread.
    // SAFETY: the output buffer is a correctly sized `proc_bsdinfo` that
    // stays alive across the call, and the return value is checked to equal
    // the struct size before any field is read.
    // AN TOÀN: buffer ra là `proc_bsdinfo` đúng kích thước, còn sống trong
    // suốt lời gọi, và giá trị trả về được kiểm bằng kích thước struct trước
    // khi đọc field.
    #[allow(unsafe_code)]
    let info = unsafe {
        let read = libc::proc_pidinfo(
            pid,
            libc::PROC_PIDTBSDINFO,
            0,
            info.as_mut_ptr().cast(),
            expected_size,
        );
        if read != expected_size {
            return None;
        }
        info.assume_init()
    };
    Some(ProcessScanSnapshot {
        parent_pid: (info.pbi_ppid != 0).then_some(info.pbi_ppid),
        process_group: (info.pbi_pgid != 0).then_some(info.pbi_pgid),
        start_time_seconds: info.pbi_start_tvsec,
        start_time_subseconds: info.pbi_start_tvusec,
        is_zombie: info.pbi_status == libc::SZOMB,
    })
}

#[cfg(target_os = "macos")]
pub(in crate::run::implementation) fn macos_process_snapshots()
-> Result<HashMap<u32, ProcessScanSnapshot>> {
    let mut process_count = macos_process_count()?;
    let pids = (0..PROCESS_SNAPSHOT_RETRIES)
        .find_map(|_| {
            let capacity = usize::try_from(process_count)
                .ok()?
                .checked_add(PROCESS_SNAPSHOT_CAPACITY_SLACK)?;
            let capacity_bytes =
                libc::c_int::try_from(capacity.checked_mul(std::mem::size_of::<libc::pid_t>())?)
                    .ok()?;
            let mut pids = vec![0 as libc::pid_t; capacity];
            // SAFETY: proc_listpids writes into a Vec buffer of `capacity`
            // pid_t sized via with_capacity; the returned byte count is checked
            // before the length is set, so only initialised pids are read.
            // AN TOÀN: proc_listpids ghi vào buffer Vec đủ `capacity` phần tử
            // pid_t; số byte trả về được kiểm trước khi set_len nên chỉ đọc pid
            // đã khởi tạo.
            #[allow(unsafe_code)]
            let count = unsafe { libc::proc_listallpids(pids.as_mut_ptr().cast(), capacity_bytes) };
            if count < 0 {
                return None;
            }
            if count as usize >= capacity {
                process_count = macos_process_count().ok()?;
                return None;
            }
            pids.truncate(count as usize);
            Some(pids)
        })
        .context("macOS process list changed while the process guard was scanning")?;

    let mut snapshots = HashMap::with_capacity(pids.len());
    for pid in pids {
        let Ok(pid) = u32::try_from(pid) else {
            continue;
        };
        if let Some(snapshot) = macos_process_snapshot(pid) {
            snapshots.insert(pid, snapshot);
        }
    }
    Ok(snapshots)
}

#[cfg(target_os = "macos")]
fn macos_process_count() -> Result<libc::c_int> {
    // SAFETY: a null buffer with size 0 is the documented size query; no
    // pointer is dereferenced and the returned count is validated.
    // AN TOÀN: buffer null với size 0 là truy vấn kích thước theo tài liệu;
    // không deref con trỏ và kết quả được kiểm.
    #[allow(unsafe_code)]
    let count = unsafe { libc::proc_listallpids(std::ptr::null_mut(), 0) };
    if count < 1 {
        anyhow::bail!("cannot enumerate macOS processes for the process guard");
    }
    Ok(count)
}

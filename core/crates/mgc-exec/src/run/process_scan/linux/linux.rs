//! Linux `/proc` process snapshots — đọc identity bền vững và xử lý race PID exit.
//! (Owner Linux cho parser stat, PID reuse và fail-closed lỗi đọc process còn sống.)

use super::super::types::ProcessScanSnapshot;
use anyhow::{Context, Result};
use std::path::Path;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

#[cfg(target_os = "linux")]
pub(in crate::run::implementation) fn linux_process_snapshots()
-> Result<std::collections::HashMap<u32, ProcessScanSnapshot>> {
    let mut snapshots = std::collections::HashMap::new();
    for entry in std::fs::read_dir("/proc").context("cannot enumerate Linux process table")? {
        let entry = entry.context("cannot read Linux process-table entry")?;
        let file_name = entry.file_name();
        let Some(pid) = file_name.to_str().and_then(|name| name.parse::<u32>().ok()) else {
            continue;
        };
        let stat_path = entry.path().join("stat");
        let stat = match std::fs::read_to_string(&stat_path) {
            Ok(stat) => stat,
            Err(_) if linux_proc_process_dir_is_gone(&entry.path())? => continue,
            Err(error) => {
                let kind = error.kind();
                let os_error = error.raw_os_error();
                return Err(error).with_context(|| {
                    format!(
                        "cannot read Linux process identity for PID {pid} (kind={:?}, os_error={:?})",
                        kind, os_error
                    )
                });
            }
        };
        let Some(snapshot) = parse_linux_process_stat_entry(pid, &stat, &entry.path())? else {
            continue;
        };
        snapshots.insert(pid, snapshot);
    }
    Ok(snapshots)
}

#[cfg(target_os = "linux")]
pub(in crate::run::implementation) fn linux_process_stat_snapshot(
    pid: u32,
) -> Result<Option<ProcessScanSnapshot>> {
    let stat_path = format!("/proc/{pid}/stat");
    let process_dir = Path::new("/proc").join(pid.to_string());
    let stat = match std::fs::read_to_string(&stat_path) {
        Ok(stat) => stat,
        Err(_) if linux_proc_process_dir_is_gone(&process_dir)? => {
            return Ok(None);
        }
        Err(error) => {
            let kind = error.kind();
            let os_error = error.raw_os_error();
            return Err(error).with_context(|| {
                format!(
                    "cannot refresh Linux process identity for PID {pid} (kind={:?}, os_error={:?})",
                    kind, os_error
                )
            });
        }
    };
    parse_linux_process_stat_entry(pid, &stat, &process_dir)
}

#[cfg(target_os = "linux")]
fn parse_linux_process_stat_entry(
    pid: u32,
    stat: &str,
    process_dir: &Path,
) -> Result<Option<ProcessScanSnapshot>> {
    if let Some(snapshot) = parse_linux_process_stat(pid, stat) {
        return Ok(Some(snapshot));
    }
    // A process may exit after stat was opened but before its contents are parsed.
    // Nếu process thoát sau khi mở stat nhưng trước khi parse xong, chỉ bỏ qua khi PID dir đã mất.
    if linux_proc_process_dir_is_gone(process_dir)? {
        return Ok(None);
    }
    anyhow::bail!("cannot parse Linux process identity for PID {pid}");
}

#[cfg(target_os = "linux")]
fn linux_proc_process_dir_is_gone(process_dir: &Path) -> Result<bool> {
    // `/proc/<pid>/stat` may disappear after directory enumeration while the
    // kernel reports a transient read error. Skip only if the PID directory is
    // gone too; a still-present but unreadable process remains fail-closed.
    // `/proc/<pid>/stat` có thể biến mất sau khi liệt kê thư mục và kernel trả
    // lỗi đọc tạm thời. Chỉ bỏ qua khi thư mục PID cũng đã biến mất; process
    // còn tồn tại nhưng không đọc được vẫn bị fail-closed.
    match std::fs::metadata(process_dir) {
        Ok(_) => Ok(false),
        Err(metadata_error)
            if metadata_error.kind() == std::io::ErrorKind::NotFound
                || metadata_error.raw_os_error() == Some(libc::ESRCH) =>
        {
            Ok(true)
        }
        Err(metadata_error) => Err(metadata_error)
            .with_context(|| format!("cannot verify Linux proc entry {}", process_dir.display())),
    }
}

#[cfg(target_os = "linux")]
fn parse_linux_process_stat(pid: u32, stat: &str) -> Option<ProcessScanSnapshot> {
    // Linux comm is parenthesized and may itself contain spaces or parentheses.
    // (Tên comm của Linux nằm trong ngoặc và có thể chứa khoảng trắng hoặc ngoặc.)
    let open_paren = stat.find('(')?;
    let close_paren = stat.rfind(')')?;
    let reported_pid = stat[..open_paren].trim().parse::<u32>().ok()?;
    if reported_pid != pid || close_paren <= open_paren {
        return None;
    }
    let mut fields = stat[close_paren + 1..].split_whitespace();
    let state = fields.next()?;
    let parent_pid = fields.next()?.parse::<u32>().ok()?;
    let process_group = fields.next()?.parse::<u32>().ok()?;
    // starttime is field 22; after state, parent PID, and process group, it is offset 16.
    // starttime là trường 22; sau state, parent PID và process group thì offset là 16.
    let process_start_ticks = fields.nth(16)?.parse::<u64>().ok()?;
    Some(ProcessScanSnapshot {
        parent_pid: (parent_pid != 0).then_some(parent_pid),
        process_group: (process_group != 0).then_some(process_group),
        process_start_ticks,
        is_zombie: matches!(state, "Z" | "X"),
    })
}

/// The monitor owner, exposed for the Linux snapshot tests that compare
/// identities through it.
/// Owner monitor, phơi cho test snapshot Linux so sánh identity qua đó.
#[cfg(test)]
pub(in crate::run::implementation::process_scan) use super::super::monitor as monitor_owner;

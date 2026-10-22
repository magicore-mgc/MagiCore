//! `process_snapshot.rs` — platform process snapshot acquisition.
//! Thu thập snapshot process theo nền tảng.

use super::super::super::super::ProcessScanSnapshot;
#[cfg(target_os = "linux")]
use super::super::super::super::linux_process_snapshots;
#[cfg(target_os = "macos")]
use super::super::super::super::macos_process_snapshots;
use anyhow::Result;
#[cfg(all(unix, not(target_os = "macos"), not(target_os = "linux")))]
use sysinfo::{Pid, ProcessStatus};
#[cfg(all(unix, not(target_os = "macos"), not(target_os = "linux")))]
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate};

#[cfg(unix)]
pub(in crate::run::implementation::process_scan::monitor::implementation) fn collect_process_snapshots(
    _system: &mut sysinfo::System,
) -> Result<std::collections::HashMap<u32, ProcessScanSnapshot>> {
    let process_snapshots = {
        #[cfg(target_os = "macos")]
        {
            macos_process_snapshots()?
        }
        #[cfg(target_os = "linux")]
        {
            linux_process_snapshots()?
        }
        #[cfg(all(not(target_os = "macos"), not(target_os = "linux")))]
        {
            _system.refresh_processes_specifics(
                ProcessesToUpdate::All,
                true,
                ProcessRefreshKind::nothing(),
            );
            _system
                .processes()
                .iter()
                .map(|(pid, process)| {
                    (
                        pid.as_u32(),
                        ProcessScanSnapshot {
                            parent_pid: process.parent().map(Pid::as_u32),
                            process_group: None,
                            start_time_seconds: process.start_time(),
                            start_time_subseconds: 0,
                            is_zombie: process.status() == ProcessStatus::Zombie,
                        },
                    )
                })
                .collect::<std::collections::HashMap<_, _>>()
        }
    };
    Ok(process_snapshots)
}

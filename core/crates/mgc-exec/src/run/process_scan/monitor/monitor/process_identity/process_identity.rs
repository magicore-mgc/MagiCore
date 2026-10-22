//! `process_identity.rs` — process snapshots and PID identity.
//! Snapshot process và identity PID.

use super::super::super::super::ProcessScanSnapshot;
#[cfg(target_os = "linux")]
use super::super::super::super::linux_process_stat_snapshot;
#[cfg(target_os = "macos")]
use super::super::super::super::macos_process_snapshot;
use anyhow::Result;

pub(in crate::run::implementation::process_scan::monitor::implementation) fn current_process_scan_snapshot(
    pid: u32,
    system: &sysinfo::System,
) -> Result<Option<ProcessScanSnapshot>> {
    #[cfg(target_os = "macos")]
    {
        let _ = system;
        Ok(macos_process_snapshot(pid))
    }
    #[cfg(target_os = "linux")]
    {
        let _ = system;
        linux_process_stat_snapshot(pid)
    }
    #[cfg(all(not(target_os = "macos"), not(target_os = "linux")))]
    {
        let Some(process) = system.process(sysinfo::Pid::from_u32(pid)) else {
            return Ok(None);
        };
        Ok(Some(ProcessScanSnapshot {
            parent_pid: process.parent().map(sysinfo::Pid::as_u32),
            process_group: None,
            start_time_seconds: process.start_time(),
            start_time_subseconds: 0,
            is_zombie: process.status() == sysinfo::ProcessStatus::Zombie,
        }))
    }
}

#[cfg(unix)]
pub(in crate::run::implementation::process_scan::monitor::implementation) fn live_descendant_identities(
    root_pid: u32,
    process_snapshots: &std::collections::HashMap<u32, ProcessScanSnapshot>,
) -> Vec<(sysinfo::Pid, ProcessScanSnapshot)> {
    let mut children_by_parent = std::collections::HashMap::<u32, Vec<u32>>::new();
    for (pid, snapshot) in process_snapshots {
        if let Some(parent) = snapshot.parent_pid {
            children_by_parent.entry(parent).or_default().push(*pid);
        }
    }

    let mut frontier = vec![root_pid];
    let mut seen = std::collections::HashSet::from([root_pid]);
    let mut descendants = Vec::new();
    while let Some(parent_pid) = frontier.pop() {
        if let Some(children) = children_by_parent.get(&parent_pid) {
            for child_pid in children {
                if !seen.insert(*child_pid) {
                    continue;
                }
                let Some(snapshot) = process_snapshots.get(child_pid).copied() else {
                    continue;
                };
                // A zombie can no longer spawn or resolve anything, but its
                // still-live children must remain reachable in the tree walk.
                frontier.push(*child_pid);
                if !snapshot.is_zombie {
                    descendants.push((sysinfo::Pid::from_u32(*child_pid), snapshot));
                }
            }
        }
    }
    descendants
}

#[cfg(unix)]
pub(in crate::run::implementation::process_scan::monitor::implementation) fn process_identity_is_live(
    initial: ProcessScanSnapshot,
    current: ProcessScanSnapshot,
) -> bool {
    if initial.is_zombie || current.is_zombie {
        return false;
    }
    #[cfg(target_os = "linux")]
    {
        initial.process_start_ticks == current.process_start_ticks
    }
    #[cfg(not(target_os = "linux"))]
    {
        initial.start_time_seconds == current.start_time_seconds
            && initial.start_time_subseconds == current.start_time_subseconds
    }
}

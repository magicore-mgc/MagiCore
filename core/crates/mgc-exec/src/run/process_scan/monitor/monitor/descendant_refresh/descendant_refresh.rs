//! `descendant_refresh.rs` — refresh current descendant process identities.
//! Làm mới identity process của hậu duệ hiện tại.

use super::super::super::super::super::termination::process_group_id;
use super::super::super::super::ProcessScanSnapshot;
use super::super::{
    current_process_scan_snapshot, live_descendant_identities, process_identity_is_live,
};
use anyhow::Result;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, UpdateKind};

#[cfg(unix)]
type LiveDescendantRefresh = Option<(Vec<sysinfo::Pid>, Vec<(sysinfo::Pid, ProcessScanSnapshot)>)>;

#[cfg(unix)]
pub(in crate::run::implementation::process_scan::monitor::implementation) fn refresh_live_descendant_pids(
    root_pid: u32,
    root_snapshot: Option<ProcessScanSnapshot>,
    process_snapshots: &std::collections::HashMap<u32, ProcessScanSnapshot>,
    system: &mut sysinfo::System,
    monitored_process_pids: &mut std::collections::HashSet<sysinfo::Pid>,
    observed_process_groups: &mut std::collections::HashSet<u32>,
) -> Result<LiveDescendantRefresh> {
    observed_process_groups.insert(root_pid);
    if let Some(root_group) = root_snapshot.and_then(|snapshot| snapshot.process_group) {
        observed_process_groups.insert(root_group);
    }
    // Build descendants from lightweight process metadata, then read command lines only there.
    // Dựng cây con từ metadata nhẹ, rồi chỉ đọc command line của các hậu duệ này.
    let descendant_identities = live_descendant_identities(root_pid, process_snapshots);
    if descendant_identities.is_empty() {
        let stale_process_pids = monitored_process_pids.drain().collect::<Vec<_>>();
        if !stale_process_pids.is_empty() {
            system.refresh_processes_specifics(
                ProcessesToUpdate::Some(&stale_process_pids),
                true,
                ProcessRefreshKind::nothing(),
            );
        }
        return Ok(None);
    }
    let descendant_pids = descendant_identities
        .iter()
        .map(|(pid, _)| *pid)
        .collect::<Vec<_>>();
    let current_process_pids = descendant_pids
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();
    let stale_process_pids = monitored_process_pids
        .difference(&current_process_pids)
        .copied()
        .collect::<Vec<_>>();
    if !stale_process_pids.is_empty() {
        system.refresh_processes_specifics(
            ProcessesToUpdate::Some(&stale_process_pids),
            true,
            ProcessRefreshKind::nothing(),
        );
    }
    let mut process_refresh_pids = descendant_pids.clone();
    process_refresh_pids.push(sysinfo::Pid::from_u32(root_pid));
    system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&process_refresh_pids),
        false,
        ProcessRefreshKind::nothing().with_cmd(UpdateKind::Always),
    );
    monitored_process_pids.clone_from(&current_process_pids);
    let mut live_descendant_pids = Vec::with_capacity(descendant_identities.len());
    for (pid, snapshot) in &descendant_identities {
        let Some(current) = current_process_scan_snapshot(pid.as_u32(), system)? else {
            continue;
        };
        if !process_identity_is_live(*snapshot, current) {
            continue;
        }
        let process_group = match current.process_group {
            Some(process_group) => Some(process_group),
            None => process_group_id(pid.as_u32())?,
        };
        if let Some(process_group) = process_group {
            observed_process_groups.insert(process_group);
        }
        live_descendant_pids.push(*pid);
    }
    Ok(Some((live_descendant_pids, descendant_identities)))
}

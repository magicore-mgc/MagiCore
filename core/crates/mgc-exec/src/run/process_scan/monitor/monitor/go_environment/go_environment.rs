//! `go_environment.rs` — scoped Go environment read and identity recheck.
//! Đọc environment Go theo scope và kiểm tra identity lại.

use super::super::super::super::super::process_basename;
use super::super::super::super::ProcessScanSnapshot;
use super::super::{current_process_scan_snapshot, process_identity_is_live};
use anyhow::Result;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, UpdateKind};

#[cfg(unix)]
pub(in crate::run::implementation::process_scan::monitor::implementation) fn refresh_go_processes(
    live_descendant_pids: Vec<sysinfo::Pid>,
    descendant_identities: &[(sysinfo::Pid, ProcessScanSnapshot)],
    system: &mut sysinfo::System,
) -> Result<Vec<sysinfo::Pid>> {
    // Read policy variables only from Go processes inside the monitored tree.
    // Chỉ đọc biến policy từ process Go nằm trong cây đang giám sát.
    let go_pids = live_descendant_pids
        .iter()
        .filter_map(|pid| {
            system
                .process(*pid)
                .filter(|process| process_basename(&process.name().to_string_lossy()) == "go")
                .map(|_| *pid)
        })
        .collect::<Vec<_>>();
    if !go_pids.is_empty() {
        system.refresh_processes_specifics(
            ProcessesToUpdate::Some(&go_pids),
            false,
            ProcessRefreshKind::nothing().with_environ(UpdateKind::Always),
        );
        // Reading /proc/<pid>/environ can race with a short-lived Go command
        // exiting. Refresh identity/status after the environment read so an
        // exited process with an empty environment is not treated as a live
        // Go invocation missing its required offline proof.
        system.refresh_processes_specifics(
            ProcessesToUpdate::Some(&go_pids),
            false,
            ProcessRefreshKind::nothing(),
        );
    }

    let go_pid_set = go_pids
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();
    let mut go_pids_live_after_env = std::collections::HashSet::new();
    for (pid, initial) in descendant_identities {
        if !go_pid_set.contains(pid) {
            continue;
        }
        let Some(current) = current_process_scan_snapshot(pid.as_u32(), system)? else {
            continue;
        };
        if process_identity_is_live(*initial, current) {
            go_pids_live_after_env.insert(*pid);
        }
    }
    let live_descendant_pids = live_descendant_pids
        .into_iter()
        .filter(|pid| !go_pids.contains(pid) || go_pids_live_after_env.contains(pid))
        .collect::<Vec<_>>();
    Ok(live_descendant_pids)
}

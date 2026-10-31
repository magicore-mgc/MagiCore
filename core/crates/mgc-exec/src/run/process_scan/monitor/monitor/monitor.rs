//! `monitor.rs` — ordered Unix process scan orchestration.
//! điều phối các bước scan process Unix theo thứ tự

#[cfg(all(unix, test))]
use super::super::super::super::ForbiddenProcess;
#[cfg(unix)]
use super::super::super::super::{CompatAuditContext, RootExitScan};
#[cfg(not(any(unix, windows)))]
use super::super::super::super::{ForbiddenProcess, ensure_process_inspection_available};
#[cfg(unix)]
use super::{
    collect_process_snapshots, find_first_forbidden, refresh_go_processes,
    refresh_live_descendant_pids, refresh_live_observed_process_groups, scan_after_root_exit,
};
#[cfg(unix)]
use crate::allowlist::ExecutionScope;
use anyhow::Result;
use std::path::Path;

#[cfg(all(unix, test))]
pub(in crate::run::implementation::process_scan::monitor::implementation) fn find_forbidden_descendant(
    root_pid: u32,
    exempt: &[&str],
    shadow_dir: &Path,
) -> Result<Option<ForbiddenProcess>> {
    find_forbidden_descendant_with_scope(root_pid, exempt, shadow_dir, ExecutionScope::Install)
}

#[cfg(all(unix, test))]
pub(in crate::run::implementation::process_scan::monitor::implementation) fn find_forbidden_descendant_with_scope(
    root_pid: u32,
    exempt: &[&str],
    shadow_dir: &Path,
    execution_scope: ExecutionScope,
) -> Result<Option<ForbiddenProcess>> {
    let mut observed_process_groups = std::collections::HashSet::from([root_pid]);
    let mut monitored_process_pids = std::collections::HashSet::new();
    find_forbidden_descendant_tracking(
        root_pid,
        &mut sysinfo::System::new(),
        &mut monitored_process_pids,
        exempt,
        shadow_dir,
        "",
        &mut observed_process_groups,
        None,
        false,
        execution_scope,
    )
    .map(RootExitScan::into_forbidden)
}

/// Keep scan policy separate from mutable OS state so every stage exposes its security inputs.
/// Tách policy khỏi trạng thái OS mutable để mỗi stage thể hiện rõ đầu vào bảo mật.
#[cfg(unix)]
#[allow(clippy::too_many_arguments)]
pub(in crate::run::implementation) fn find_forbidden_descendant_tracking(
    root_pid: u32,
    system: &mut sysinfo::System,
    monitored_process_pids: &mut std::collections::HashSet<sysinfo::Pid>,
    exempt: &[&str],
    shadow_dir: &Path,
    tracking_token: &str,
    observed_process_groups: &mut std::collections::HashSet<u32>,
    compat_audit: Option<&CompatAuditContext<'_>>,
    root_reaped: bool,
    execution_scope: ExecutionScope,
) -> Result<RootExitScan> {
    let process_snapshots = collect_process_snapshots(system)?;
    refresh_live_observed_process_groups(observed_process_groups)?;
    let root_snapshot = process_snapshots.get(&root_pid).copied();
    let root_is_visible = !root_reaped && root_snapshot.is_some_and(|snapshot| !snapshot.is_zombie);
    if !root_is_visible {
        return scan_after_root_exit(
            root_pid,
            tracking_token,
            observed_process_groups,
            compat_audit,
            exempt,
            shadow_dir,
            execution_scope,
        );
    }
    let Some((live_descendant_pids, descendant_identities)) = refresh_live_descendant_pids(
        root_pid,
        root_snapshot,
        &process_snapshots,
        system,
        monitored_process_pids,
        observed_process_groups,
    )?
    else {
        return Ok(RootExitScan::Running);
    };
    let live_descendant_pids =
        refresh_go_processes(live_descendant_pids, &descendant_identities, system)?;
    Ok(
        match find_first_forbidden(
            &live_descendant_pids,
            system,
            exempt,
            shadow_dir,
            compat_audit,
            execution_scope,
        )? {
            Some(found) => RootExitScan::Forbidden(found),
            None => RootExitScan::Running,
        },
    )
}

#[cfg(not(any(unix, windows)))]
fn find_forbidden_descendant(
    _root_pid: u32,
    _exempt: &[&str],
    _shadow_dir: &Path,
) -> Result<Option<ForbiddenProcess>> {
    ensure_process_inspection_available(true, false)?;
    Ok(None)
}

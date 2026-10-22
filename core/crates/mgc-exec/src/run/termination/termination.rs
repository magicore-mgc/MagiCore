//! Coordinate bounded Unix TERM/KILL sweeps over observed child process groups.
//! Điều phối các lượt TERM/KILL Unix có giới hạn trên process group đã quan sát.

use super::super::find_forbidden_descendant_tracking;
use super::error::{TerminationFailures, TerminationPhase};
use super::process_group_probe::process_groups_have_descendants;
use super::process_group_signal::{signal_process_group_ids, signal_process_groups};
use crate::allowlist::ExecutionScope;
use std::path::Path;
use std::time::{Duration, Instant};

/// Grace window between the SIGTERM and SIGKILL sweep over the process group.
/// Khoảng chờ giữa lượt SIGTERM và SIGKILL trên process group.
#[cfg(unix)]
const TERM_TO_KILL_GRACE_MS: u64 = 100;

#[cfg(all(unix, test))]
pub(in crate::run::implementation) fn terminate_process_tree(
    root_pid: u32,
    observed_process_groups: &std::collections::HashSet<u32>,
) -> std::result::Result<(), super::TerminationError> {
    let mut failures = TerminationFailures::default();
    let mut groups = observed_process_groups.clone();
    groups.insert(root_pid);
    signal_process_group_ids(&groups, libc::SIGTERM, &mut failures);
    std::thread::sleep(Duration::from_millis(TERM_TO_KILL_GRACE_MS));
    signal_process_group_ids(&groups, libc::SIGKILL, &mut failures);
    failures.into_result()
}

#[cfg(unix)]
// Keep the process cache, signal targets, and policy inputs explicit across termination.
// Giữ cache process, nhóm tín hiệu và policy tách bạch trong nhánh dừng cây process.
#[allow(clippy::too_many_arguments)]
pub(in crate::run::implementation) fn terminate_process_tree_tracked(
    root_pid: u32,
    system: &mut sysinfo::System,
    monitored_process_pids: &mut std::collections::HashSet<sysinfo::Pid>,
    observed_process_groups: &mut std::collections::HashSet<u32>,
    tracking_token: &str,
    exempt: &[&str],
    shadow_dir: &Path,
    root_reaped: bool,
    execution_scope: ExecutionScope,
) -> std::result::Result<(), super::TerminationError> {
    let mut failures = TerminationFailures::default();
    signal_process_groups(
        root_pid,
        observed_process_groups,
        libc::SIGTERM,
        root_reaped,
        &mut failures,
    );
    let mut term_signaled_groups = observed_process_groups.clone();
    let term_deadline = Instant::now() + Duration::from_millis(TERM_TO_KILL_GRACE_MS);
    loop {
        let previous_groups = observed_process_groups.len();
        if let Err(source) = find_forbidden_descendant_tracking(
            root_pid,
            system,
            monitored_process_pids,
            exempt,
            shadow_dir,
            tracking_token,
            observed_process_groups,
            None,
            root_reaped,
            execution_scope,
        ) {
            failures.process_scan(TerminationPhase::TermSweep, source);
        }
        let new_groups = observed_process_groups
            .difference(&term_signaled_groups)
            .copied()
            .collect::<std::collections::HashSet<_>>();
        signal_process_group_ids(&new_groups, libc::SIGTERM, &mut failures);
        term_signaled_groups.extend(new_groups);
        if Instant::now() >= term_deadline {
            break;
        }
        let sleep_for = if observed_process_groups.len() != previous_groups {
            Duration::from_millis(1)
        } else {
            Duration::from_millis(10)
        };
        std::thread::sleep(sleep_for.min(term_deadline.saturating_duration_since(Instant::now())));
    }

    signal_process_groups(
        root_pid,
        observed_process_groups,
        libc::SIGKILL,
        root_reaped,
        &mut failures,
    );
    let kill_deadline = Instant::now() + Duration::from_millis(TERM_TO_KILL_GRACE_MS);
    loop {
        if let Err(source) = find_forbidden_descendant_tracking(
            root_pid,
            system,
            monitored_process_pids,
            exempt,
            shadow_dir,
            tracking_token,
            observed_process_groups,
            None,
            root_reaped,
            execution_scope,
        ) {
            failures.process_scan(TerminationPhase::KillSweep, source);
        }
        signal_process_groups(
            root_pid,
            observed_process_groups,
            libc::SIGKILL,
            root_reaped,
            &mut failures,
        );
        let descendants_remain =
            match process_groups_have_descendants(system, observed_process_groups, root_pid) {
                Ok(remain) => remain,
                Err(source) => {
                    failures.process_scan(TerminationPhase::KillSweep, source.into());
                    true
                }
            };
        if !descendants_remain || Instant::now() >= kill_deadline {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    failures.into_result()
}

#[cfg(all(unix, test))]
#[path = "test/mod.rs"]
mod tests;

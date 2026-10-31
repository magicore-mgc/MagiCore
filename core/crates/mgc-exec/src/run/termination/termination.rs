//! Coordinate bounded Unix TERM/KILL sweeps over observed child process groups.
//! Điều phối các lượt TERM/KILL Unix có giới hạn trên process group đã quan sát.

use super::super::find_forbidden_descendant_tracking;
use super::error::{TerminationFailures, TerminationPhase};
use super::process_group_probe::process_groups_have_descendants;
use super::process_group_signal::{signal_process_group_ids, signal_process_groups};
use crate::allowlist::ExecutionScope;
use crate::run::implementation::RootExitScan;
use std::path::Path;
use std::time::{Duration, Instant};

/// Grace window between the SIGTERM and SIGKILL sweep over the process group.
/// Khoảng chờ giữa lượt SIGTERM và SIGKILL trên process group.
#[cfg(unix)]
const TERM_TO_KILL_GRACE_MS: u64 = 100;

/// Pause between sweeps while the observed group set is unchanged.
/// Khoảng nghỉ giữa các lượt khi tập process group quan sát không đổi.
#[cfg(unix)]
const TERM_SWEEP_STABLE_INTERVAL_MS: u64 = 10;

/// Pause between sweeps while the observed group set is still growing.
/// Khoảng nghỉ giữa các lượt khi tập process group vẫn còn tăng.
#[cfg(unix)]
const TERM_SWEEP_GROWING_INTERVAL_MS: u64 = 1;

/// Pause between SIGKILL sweeps over surviving descendants.
/// Khoảng nghỉ giữa các lượt SIGKILL trên descendant còn sống.
#[cfg(unix)]
const KILL_SWEEP_INTERVAL_MS: u64 = 10;

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
        let tree_is_gone = scan_tracked_tree(
            root_pid,
            system,
            monitored_process_pids,
            exempt,
            shadow_dir,
            tracking_token,
            observed_process_groups,
            root_reaped,
            execution_scope,
            TerminationPhase::TermSweep,
            &mut failures,
        );
        let new_groups = observed_process_groups
            .difference(&term_signaled_groups)
            .copied()
            .collect::<std::collections::HashSet<_>>();
        signal_process_group_ids(&new_groups, libc::SIGTERM, &mut failures);
        term_signaled_groups.extend(new_groups);
        // A gone root with no tracked descendant has nothing left to sweep. Stop
        // here instead of rescanning the whole process table for the full grace
        // window (the old loop produced one diagnostics-bounded error per scan).
        // (Root đã mất và không còn descendant thì không còn gì để quét. Dừng ở
        // đây thay vì scan lại toàn bộ bảng process suốt grace window — vòng lặp
        // cũ tạo một lỗi cho mỗi lượt scan.)
        if tree_is_gone || Instant::now() >= term_deadline {
            break;
        }
        let sleep_for = if observed_process_groups.len() != previous_groups {
            TERM_SWEEP_GROWING_INTERVAL_MS
        } else {
            TERM_SWEEP_STABLE_INTERVAL_MS
        };
        std::thread::sleep(
            Duration::from_millis(sleep_for)
                .min(term_deadline.saturating_duration_since(Instant::now())),
        );
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
        scan_tracked_tree(
            root_pid,
            system,
            monitored_process_pids,
            exempt,
            shadow_dir,
            tracking_token,
            observed_process_groups,
            root_reaped,
            execution_scope,
            TerminationPhase::KillSweep,
            &mut failures,
        );
        signal_process_groups(
            root_pid,
            observed_process_groups,
            libc::SIGKILL,
            root_reaped,
            &mut failures,
        );
        // A vanished root already exits this check, so a completed tree never
        // waits out the deadline. (Root đã mất khiến phép dò group luôn báo sạch,
        // nên cây đã xong không phải chờ hết deadline.)
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
        std::thread::sleep(Duration::from_millis(KILL_SWEEP_INTERVAL_MS));
    }
    failures.into_result()
}

/// Scan the tracked tree and report whether it is completely gone.
/// Scan cây có tracking và báo cây đã kết thúc hoàn toàn hay chưa.
///
/// Only genuine inspection failures are recorded. A root that already exited is a
/// completed tree, not a sweep failure; a forbidden descendant found mid-sweep is
/// already being signalled and is not a new failure either.
/// (Chỉ ghi nhận lỗi inspection thật. Root đã thoát là cây hoàn tất, không phải
/// lỗi lượt quét; descendant bị cấm phát hiện giữa lượt quét cũng đã bị gửi
/// signal nên không phải lỗi mới.)
#[cfg(unix)]
#[allow(clippy::too_many_arguments)]
fn scan_tracked_tree(
    root_pid: u32,
    system: &mut sysinfo::System,
    monitored_process_pids: &mut std::collections::HashSet<sysinfo::Pid>,
    exempt: &[&str],
    shadow_dir: &Path,
    tracking_token: &str,
    observed_process_groups: &mut std::collections::HashSet<u32>,
    root_reaped: bool,
    execution_scope: ExecutionScope,
    phase: TerminationPhase,
    failures: &mut TerminationFailures,
) -> bool {
    match find_forbidden_descendant_tracking(
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
        Ok(RootExitScan::Exited) => true,
        Ok(RootExitScan::Running | RootExitScan::Forbidden(_)) => false,
        Err(source) => {
            failures.process_scan(phase, source);
            false
        }
    }
}

#[cfg(all(unix, test))]
#[path = "test/mod.rs"]
mod tests;

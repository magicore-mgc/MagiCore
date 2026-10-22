//! Recheck observed process groups before sending Unix signals.
//! Kiểm tra lại process group đã quan sát trước khi gửi signal Unix.

use super::super::error::{TerminationFailures, TerminationPhase};
use super::super::process_group_probe::{process_group_id, process_group_is_live};
use super::super::process_signal_plan::process_signal_plan;

#[cfg(unix)]
pub(in crate::run::implementation::termination) fn signal_process_groups(
    root_pid: u32,
    process_groups: &std::collections::HashSet<u32>,
    signal: libc::c_int,
    root_reaped: bool,
    failures: &mut TerminationFailures,
) {
    let mut candidate_groups = process_groups.clone();
    let root_group_id = if root_reaped {
        None
    } else {
        match process_group_id(root_pid) {
            Ok(group) => group,
            Err(source) => {
                failures.process_scan(TerminationPhase::TermSweep, source.into());
                None
            }
        }
    };
    if !root_reaped {
        candidate_groups.insert(root_pid);
        if let Some(root_group_id) = root_group_id {
            candidate_groups.insert(root_group_id);
        }
    }
    let mut live_process_groups = std::collections::HashSet::new();
    for group in candidate_groups {
        match process_group_is_live(group) {
            Ok(true) => {
                live_process_groups.insert(group);
            }
            Ok(false) => {}
            Err(source) => failures.process_group_probe(group, source),
        }
    }
    let plan = process_signal_plan(
        root_pid,
        process_groups,
        &live_process_groups,
        root_group_id,
        root_reaped,
    );
    signal_process_group_ids(&plan.process_groups, signal, failures);
    if plan.signal_root_directly {
        // The root PID remains reserved until `Child::wait` reaps it.
        // PID root chưa thể được tái sử dụng cho đến khi `Child::wait` reap.
        // SAFETY: this PID belongs to our unreaped direct child.
        // AN TOÀN: PID này thuộc child trực tiếp do runner spawn và chưa reap.
        #[allow(unsafe_code)]
        unsafe {
            match libc::pid_t::try_from(root_pid) {
                Ok(root_pid) => {
                    if libc::kill(root_pid, signal) != 0 {
                        let source = std::io::Error::last_os_error();
                        if source.raw_os_error() != Some(libc::ESRCH) {
                            failures.root_signal(root_pid as u32, signal, source);
                        }
                    }
                }
                Err(_) => failures.root_signal(
                    root_pid,
                    signal,
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "child PID does not fit the platform process identifier type",
                    ),
                ),
            }
        }
    }
}

#[cfg(unix)]
pub(in crate::run::implementation::termination) fn signal_process_group_ids(
    groups: &std::collections::HashSet<u32>,
    signal: libc::c_int,
    failures: &mut TerminationFailures,
) {
    // Every target is a process group previously observed under this runner.
    // Mọi đích đều là process group riêng đã quan sát dưới runner này.
    // SAFETY: negative PIDs target only process groups observed under this runner.
    // AN TOÀN: PID âm chỉ nhắm các process group đã được quan sát thuộc runner này.
    #[allow(unsafe_code)]
    unsafe {
        for group in groups {
            let process_group = match libc::pid_t::try_from(*group) {
                Ok(group) => group,
                Err(_) => {
                    failures.process_group_signal(
                        *group,
                        signal,
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "process-group ID does not fit the platform process identifier type",
                        ),
                    );
                    continue;
                }
            };
            match process_group_is_live(*group) {
                Ok(true) => {}
                Ok(false) => continue,
                Err(source) => {
                    failures.process_group_probe(*group, source);
                    continue;
                }
            }
            if libc::kill(-process_group, signal) != 0 {
                let source = std::io::Error::last_os_error();
                if source.raw_os_error() != Some(libc::ESRCH) {
                    failures.process_group_signal(*group, signal, source);
                }
            }
        }
    }
}

#[cfg(all(unix, test))]
#[path = "test/mod.rs"]
mod tests;

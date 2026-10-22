//! Mutable state owned by one child-process supervision request.
//! Trạng thái mutable của một yêu cầu giám sát process con.

use super::output_capture::OutputCapture;
use crate::run::implementation::{ProcessMonitor, ProcessTreeGuard};
use std::time::{Duration, Instant};

pub(in crate::run::implementation::supervision::implementation::wait_session) struct WaitSession<'a>
{
    pub(super) child: std::process::Child,
    pub(super) timeout: Option<Duration>,
    pub(super) process_monitor: ProcessMonitor<'a>,
    pub(super) output_capture: OutputCapture,
    pub(super) started: Instant,
    #[cfg(windows)]
    pub(super) process_tree_guard: Option<ProcessTreeGuard>,
    #[cfg(unix)]
    pub(super) observed_process_groups: std::collections::HashSet<u32>,
    #[cfg(unix)]
    pub(super) root_reaped: bool,
    #[cfg(unix)]
    pub(super) process_system: sysinfo::System,
    #[cfg(unix)]
    pub(super) monitored_process_pids: std::collections::HashSet<sysinfo::Pid>,
}

impl<'a> WaitSession<'a> {
    pub(super) fn new(
        mut child: std::process::Child,
        timeout: Option<Duration>,
        process_monitor: ProcessMonitor<'a>,
        process_tree_guard: ProcessTreeGuard,
    ) -> Self {
        let started = Instant::now();
        #[cfg(windows)]
        let process_tree_guard = Some(process_tree_guard);
        #[cfg(not(windows))]
        let _ = process_tree_guard;
        #[cfg(unix)]
        let observed_process_groups = std::collections::HashSet::from([child.id()]);
        #[cfg(unix)]
        let root_reaped = false;
        #[cfg(unix)]
        let process_system = sysinfo::System::new();
        #[cfg(unix)]
        let monitored_process_pids = std::collections::HashSet::<sysinfo::Pid>::new();
        let output_capture = OutputCapture::start(&mut child);

        Self {
            #[cfg(unix)]
            observed_process_groups,
            #[cfg(unix)]
            root_reaped,
            #[cfg(unix)]
            process_system,
            #[cfg(unix)]
            monitored_process_pids,
            child,
            timeout,
            process_monitor,
            output_capture,
            started,
            #[cfg(windows)]
            process_tree_guard,
        }
    }
}

//! Coordinate cancellation, inspection, root exit, and timeout per poll.
//! Điều phối cancellation, inspection, root-exit và timeout theo từng poll.

use super::super::implementation::WaitSession;
use crate::run::implementation::output::{ExecOutcome, OutputMode};
use crate::run::implementation::{ProcessMonitor, ProcessTreeGuard, WAIT_POLL_INTERVAL_MS};
use anyhow::Result;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

pub(in crate::run::implementation) fn wait_with_timeout(
    child: std::process::Child,
    timeout: Option<Duration>,
    process_monitor: ProcessMonitor<'_>,
    process_tree_guard: ProcessTreeGuard,
    mode: OutputMode,
    cancel_flag: Option<&AtomicBool>,
) -> Result<ExecOutcome> {
    let _ = mode;
    WaitSession::new(child, timeout, process_monitor, process_tree_guard).run(cancel_flag)
}

impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn run(
        &mut self,
        cancel_flag: Option<&AtomicBool>,
    ) -> Result<ExecOutcome> {
        loop {
            self.fail_if_cancelled(cancel_flag)?;
            #[cfg(unix)]
            let process_scan = self.scan_before_root_wait();
            #[cfg(unix)]
            self.reject_pre_exit_match(&process_scan)?;

            if self.child.try_wait()?.is_some() {
                #[cfg(unix)]
                {
                    self.root_reaped = true;
                    return self.finish_root_exit(process_scan);
                }
                #[cfg(windows)]
                return self.finish_root_exit();
                #[cfg(not(any(unix, windows)))]
                return Ok(self.output_capture.drain(&mut self.child, None)?.0);
            }

            if self.process_monitor.enabled {
                #[cfg(unix)]
                self.inspect_unix_processes(process_scan)?;
                #[cfg(windows)]
                if let Some(outcome) = self.inspect_windows_processes()? {
                    return Ok(outcome);
                }
                #[cfg(not(any(unix, windows)))]
                return Err(self.fail_process_inspection(
                    crate::run::implementation::process_inspection_unavailable_error(),
                    "during command execution",
                    "failing closed after process inspection failed",
                    "failing closed after process inspection failed",
                ));
            }

            self.enforce_timeout()?;
            std::thread::sleep(Duration::from_millis(WAIT_POLL_INTERVAL_MS));
        }
    }
}

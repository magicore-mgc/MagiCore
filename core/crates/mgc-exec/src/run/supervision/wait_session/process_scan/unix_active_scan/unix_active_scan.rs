//! Retry transient Unix root-visibility gaps and classify process findings.
//! Thử lại khi Unix root tạm thời không hiển thị và phân loại kết quả scan.

use super::super::super::implementation::WaitSession;
use super::super::ProcessScan;
use crate::run::implementation::{
    MonitoredRootNotVisible, WAIT_POLL_INTERVAL_MS, find_forbidden_descendant_tracking,
};
use anyhow::Result;
use std::time::Duration;

/// Retry a missing-root snapshot for 100 ms before treating it as unverifiable.
/// Thử lại snapshot thiếu root trong 100 ms trước khi xem là không thể xác minh.
#[cfg(unix)]
const ROOT_VISIBILITY_RETRIES: usize = 5;

impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn inspect_unix_processes(
        &mut self,
        mut process_scan: ProcessScan,
    ) -> Result<()> {
        let root_was_missing = process_scan
            .as_ref()
            .err()
            .is_some_and(|error| error.downcast_ref::<MonitoredRootNotVisible>().is_some());
        if root_was_missing {
            self.retry_missing_unix_root(&mut process_scan);
        }

        if process_scan
            .as_ref()
            .err()
            .is_some_and(|error| error.downcast_ref::<MonitoredRootNotVisible>().is_some())
            && self.child.try_wait()?.is_some()
        {
            self.root_reaped = true;
            process_scan = match find_forbidden_descendant_tracking(
                self.child.id(),
                &mut self.process_system,
                &mut self.monitored_process_pids,
                self.process_monitor.exempt,
                self.process_monitor.shadow_dir,
                self.process_monitor.tracking_token,
                &mut self.observed_process_groups,
                self.process_monitor.compat_audit,
                self.root_reaped,
                self.process_monitor.execution_scope,
            ) {
                Ok(scan) => Ok(scan),
                Err(error) if error.downcast_ref::<MonitoredRootNotVisible>().is_some() => Ok(None),
                Err(error) => Err(error),
            };
        }

        match process_scan {
            Ok(Some(found)) => Err(self.fail_forbidden_process(&found, true)),
            Ok(None) => Ok(()),
            Err(error) => Err(self.fail_process_inspection(
                error,
                "during command execution",
                "failing closed after process inspection failed",
                "failing closed after process inspection failed",
            )),
        }
    }

    fn retry_missing_unix_root(&mut self, process_scan: &mut ProcessScan) {
        for _ in 0..ROOT_VISIBILITY_RETRIES {
            match self.child.try_wait() {
                Ok(Some(_)) => {
                    self.root_reaped = true;
                    match self.scan_tracked_unix_descendants() {
                        Ok(scan) => {
                            *process_scan = Ok(scan);
                            break;
                        }
                        Err(error) if error.downcast_ref::<MonitoredRootNotVisible>().is_some() => {
                            *process_scan = Ok(None);
                            break;
                        }
                        Err(error) => {
                            *process_scan = Err(error);
                            break;
                        }
                    }
                }
                Ok(None) => {
                    std::thread::sleep(Duration::from_millis(WAIT_POLL_INTERVAL_MS));
                    match self.scan_tracked_unix_descendants() {
                        Ok(scan) => {
                            *process_scan = Ok(scan);
                            break;
                        }
                        Err(error) if error.downcast_ref::<MonitoredRootNotVisible>().is_some() => {
                            continue;
                        }
                        Err(error) => {
                            *process_scan = Err(error);
                            break;
                        }
                    }
                }
                Err(error) => {
                    *process_scan = Err(anyhow::Error::from(error));
                    break;
                }
            }
        }
    }

    fn scan_tracked_unix_descendants(
        &mut self,
    ) -> Result<Option<crate::run::implementation::ForbiddenProcess>> {
        find_forbidden_descendant_tracking(
            self.child.id(),
            &mut self.process_system,
            &mut self.monitored_process_pids,
            self.process_monitor.exempt,
            self.process_monitor.shadow_dir,
            self.process_monitor.tracking_token,
            &mut self.observed_process_groups,
            self.process_monitor.compat_audit,
            self.root_reaped,
            self.process_monitor.execution_scope,
        )
    }
}

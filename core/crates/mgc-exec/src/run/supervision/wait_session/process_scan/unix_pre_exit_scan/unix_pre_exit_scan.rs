//! Scan tracked Unix descendants before checking the root child status.
//! Quét descendant Unix có tracking trước khi kiểm tra trạng thái root child.

use super::super::super::implementation::WaitSession;
use super::super::super::termination::termination_failure;
use super::super::ProcessScan;
use crate::run::implementation::{POST_KILL_DRAIN_MS, find_forbidden_descendant_tracking};
use anyhow::Result;
use std::time::Duration;

impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn scan_before_root_wait(
        &mut self,
    ) -> ProcessScan {
        if self.process_monitor.enabled {
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
        } else {
            Ok(None)
        }
    }

    pub(in crate::run::implementation::supervision::implementation::wait_session) fn reject_pre_exit_match(
        &mut self,
        process_scan: &ProcessScan,
    ) -> Result<()> {
        let Ok(Some(found)) = process_scan else {
            return Ok(());
        };
        let termination_error = self.terminate_tree();
        let trigger = match self.output_capture.drain(
            &mut self.child,
            Some(Duration::from_millis(POST_KILL_DRAIN_MS)),
        ) {
            Ok((out, _)) => crate::run::implementation::forbidden_child_error(
                found,
                out.status,
                &out.stderr,
                &out.stdout,
                self.process_monitor.native_audit,
            )
            .into(),
            Err(error) => error,
        };
        Err(termination_failure(
            "rejecting a forbidden child process",
            termination_error,
            None,
            trigger,
        ))
    }
}

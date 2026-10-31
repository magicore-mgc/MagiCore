//! Revalidate Unix descendants and finish output collection after root exit.
//! Xác minh lại descendant Unix và thu output sau khi root process thoát.

use super::super::super::implementation::WaitSession;
use super::super::super::process_scan::ProcessScan;
use crate::run::implementation::supervision::implementation::error::SupervisionError;
use crate::run::implementation::{
    POST_KILL_DRAIN_MS, RootExitScan, find_forbidden_descendant_tracking,
    process_groups_have_descendants,
};
use anyhow::Result;
use std::time::Duration;

impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn finish_unix_root_exit(
        &mut self,
        process_scan: ProcessScan,
    ) -> Result<crate::run::implementation::output::ExecOutcome> {
        if self.process_monitor.enabled {
            let process_scan = self.refresh_unix_exit_scan(process_scan);
            self.reject_unix_exit_scan(process_scan)?;
            self.ensure_unix_process_group_empty()?;
        }

        if let Some(outcome) = self.drain_until_timeout()? {
            return Ok(outcome);
        }
        Ok(self.output_capture.drain(&mut self.child, None)?.0)
    }

    fn refresh_unix_exit_scan(&mut self, process_scan: ProcessScan) -> ProcessScan {
        // Re-scan after root reap to close the detached-child exit race.
        // Quét lại sau khi reap root để khép race child tách process group.
        // A scan that already observed the exited root is authoritative; only a
        // running-tree snapshot or an inspection error needs the exit re-scan.
        // (Scan đã thấy root thoát là kết quả chuẩn; chỉ snapshot cây còn chạy
        // hoặc lỗi inspection mới cần quét lại sau exit.)
        if matches!(process_scan, Ok(RootExitScan::Exited)) {
            return process_scan;
        }
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

    fn reject_unix_exit_scan(&mut self, process_scan: ProcessScan) -> Result<()> {
        match process_scan {
            Ok(RootExitScan::Forbidden(found)) => Err(self.fail_forbidden_process(&found, true)),
            // `Exited` with no tracked survivor is exactly a clean shutdown, so it
            // must not surface as an inspection failure.
            // (`Exited` không còn descendant là đúng một lần tắt sạch, nên không
            // được nổi lên như lỗi inspection.)
            Ok(RootExitScan::Exited) | Ok(RootExitScan::Running) => Ok(()),
            Err(error) => Err(self.fail_process_inspection(
                error,
                "after command exit",
                "failing closed after process inspection failed",
                "failing closed after process inspection failed",
            )),
        }
    }

    fn ensure_unix_process_group_empty(&mut self) -> Result<()> {
        match process_groups_have_descendants(
            &mut self.process_system,
            &self.observed_process_groups,
            self.child.id(),
        ) {
            Ok(false) => Ok(()),
            Err(source) => Err(self.fail_process_inspection(
                source.into(),
                "after command exit",
                "failing closed after process-group inspection failed",
                "failing closed after process-group inspection failed",
            )),
            Ok(true) => Err(self.fail_remaining_unix_group_children()),
        }
    }

    fn fail_remaining_unix_group_children(&mut self) -> anyhow::Error {
        use super::super::super::termination::{kill_child, termination_failure};

        let termination_error = self.terminate_tree();
        let child_kill = kill_child(
            &mut self.child,
            "terminating remaining process-group descendants",
        )
        .err();
        let trigger = match self.output_capture.drain(
            &mut self.child,
            Some(Duration::from_millis(POST_KILL_DRAIN_MS)),
        ) {
            Ok((out, _)) => SupervisionError::ProcessGroupDescendants { status: out.status }.into(),
            Err(error) => error,
        };
        termination_failure(
            "terminating remaining process-group descendants",
            termination_error,
            child_kill,
            trigger,
        )
    }
}

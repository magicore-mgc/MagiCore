//! Shared process-scan result and fail-closed diagnostic construction.
//! Kết quả scan dùng chung và tạo diagnostic khi fail-closed.

use super::super::implementation::WaitSession;
use super::super::termination::{kill_child, termination_failure};
use crate::run::implementation::ForbiddenProcess;
use crate::run::implementation::{POST_KILL_DRAIN_MS, forbidden_child_error};
use anyhow::Result;
use std::time::Duration;

pub(in crate::run::implementation::supervision::implementation::wait_session) type ProcessScan =
    Result<Option<ForbiddenProcess>>;

impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn fail_forbidden_process(
        &mut self,
        found: &ForbiddenProcess,
        kill_direct_child: bool,
    ) -> anyhow::Error {
        let termination_error = self.terminate_tree();
        self.release_tree_guard();
        let child_kill = kill_direct_child
            .then(|| kill_child(&mut self.child, "rejecting a forbidden child process").err())
            .flatten();
        let trigger = match self.output_capture.drain(
            &mut self.child,
            Some(Duration::from_millis(POST_KILL_DRAIN_MS)),
        ) {
            Ok((out, _)) => forbidden_child_error(
                found,
                out.status,
                &out.stderr,
                &out.stdout,
                self.process_monitor.native_audit,
            )
            .into(),
            Err(error) => error,
        };
        termination_failure(
            "rejecting a forbidden child process",
            termination_error,
            child_kill,
            trigger,
        )
    }

    pub(in crate::run::implementation::supervision::implementation::wait_session) fn fail_process_inspection(
        &mut self,
        source: anyhow::Error,
        phase: &'static str,
        termination_action: &'static str,
        child_kill_action: &'static str,
    ) -> anyhow::Error {
        let termination_error = self.terminate_tree();
        self.release_tree_guard();
        let child_kill = kill_child(&mut self.child, child_kill_action).err();
        let trigger = match self
            .output_capture
            .drain(&mut self.child, Some(Duration::from_millis(POST_KILL_DRAIN_MS)))
        {
            Ok((out, _)) => crate::run::implementation::supervision::implementation::error::SupervisionError::ProcessInspection {
                phase,
                status: out.status,
                source,
            }
            .into(),
            Err(error) => error,
        };
        termination_failure(termination_action, termination_error, child_kill, trigger)
    }
}

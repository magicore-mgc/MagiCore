//! Enforce the configured deadline and preserve the timeout diagnostic.
//! Thực thi deadline cấu hình và giữ diagnostic timeout hiện hành.

use super::super::implementation::WaitSession;
use super::super::termination::{kill_child, termination_failure};
use crate::run::implementation::{POST_KILL_DRAIN_MS, timeout_error};
use anyhow::Result;
use std::time::Duration;

impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn enforce_timeout(
        &mut self,
    ) -> Result<()> {
        let Some(timeout) = self
            .timeout
            .filter(|timeout| self.started.elapsed() >= *timeout)
        else {
            return Ok(());
        };

        let termination_error = self.terminate_tree();
        self.release_tree_guard();
        let child_kill = kill_child(&mut self.child, "enforcing the execution timeout").err();
        let trigger = match self.output_capture.drain(
            &mut self.child,
            Some(Duration::from_millis(POST_KILL_DRAIN_MS)),
        ) {
            Ok((out, _)) => timeout_error(timeout, out.status, &out.stderr, &out.stdout).into(),
            Err(error) => error,
        };
        Err(termination_failure(
            "enforcing the execution timeout",
            termination_error,
            child_kill,
            trigger,
        ))
    }

    pub(in crate::run::implementation::supervision::implementation::wait_session) fn drain_until_timeout(
        &mut self,
    ) -> Result<Option<crate::run::implementation::output::ExecOutcome>> {
        let Some(timeout) = self.timeout else {
            return Ok(None);
        };
        let remaining = timeout.saturating_sub(self.started.elapsed());
        let (out, complete) = self
            .output_capture
            .drain(&mut self.child, Some(remaining))?;
        if complete {
            return Ok(Some(out));
        }

        let termination_error = self.terminate_tree();
        self.release_tree_guard();
        let child_kill = kill_child(&mut self.child, "enforcing the execution timeout").err();
        let trigger = match self.output_capture.drain(
            &mut self.child,
            Some(Duration::from_millis(POST_KILL_DRAIN_MS)),
        ) {
            Ok((out, _)) => timeout_error(timeout, out.status, &out.stderr, &out.stdout).into(),
            Err(error) => error,
        };
        Err(termination_failure(
            "enforcing the execution timeout",
            termination_error,
            child_kill,
            trigger,
        ))
    }
}

//! Transfer the Windows Job guard to its existing root-exit cleanup owner.
//! Chuyển Windows Job guard sang owner cleanup root-exit hiện có.

use super::super::super::implementation::WaitSession;
use crate::run::implementation::output::ExecOutcome;
use crate::run::implementation::windows_job::finish_monitored_root_exit;
use crate::run::implementation::{POST_KILL_DRAIN_MS, ProcessTreeGuard};
use anyhow::Result;
use std::time::Duration;

impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn finish_windows_root_exit(
        &mut self,
    ) -> Result<ExecOutcome> {
        if self.process_monitor.enabled {
            let guard: ProcessTreeGuard = self
                .process_tree_guard
                .take()
                .expect("Windows process-tree guard transfers once at root exit");
            let child = &mut self.child;
            let output_capture = &self.output_capture;
            return finish_monitored_root_exit(
                child,
                guard,
                self.process_monitor.exempt,
                self.process_monitor.shadow_path,
                self.process_monitor.compat_audit,
                self.process_monitor.native_audit,
                self.process_monitor.execution_scope,
                |child, bounded| output_capture.drain(child, bounded),
            )
            .map_err(anyhow::Error::from);
        }

        self.release_tree_guard();
        Ok(self
            .output_capture
            .drain(
                &mut self.child,
                Some(Duration::from_millis(POST_KILL_DRAIN_MS)),
            )?
            .0)
    }
}

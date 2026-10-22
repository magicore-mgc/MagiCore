//! Inspect the Windows Job while the root command remains active.
//! Kiểm tra Windows Job khi root command còn chạy.

use super::super::super::implementation::WaitSession;
use crate::run::implementation::WindowsProcessScan;
use crate::run::implementation::output::ExecOutcome;
use crate::run::implementation::windows_job::find_forbidden_job_descendant_with_timeout;
use anyhow::Result;

impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn inspect_windows_processes(
        &mut self,
    ) -> Result<Option<ExecOutcome>> {
        let guard = self
            .process_tree_guard
            .as_ref()
            .expect("Windows process-tree guard remains owned until root-exit completion");
        let process_scan = find_forbidden_job_descendant_with_timeout(
            guard,
            self.child.id(),
            self.process_monitor.exempt,
            self.process_monitor.shadow_path,
            self.process_monitor.compat_audit,
            self.started,
            self.timeout,
            self.process_monitor.execution_scope,
            || Ok(self.child.try_wait()?.is_some()),
        );
        let process_scan = match process_scan {
            Ok(WindowsProcessScan::Clean) | Ok(WindowsProcessScan::DeadlineReached) => Ok(None),
            Ok(WindowsProcessScan::Forbidden(found)) => Ok(Some(found)),
            Ok(WindowsProcessScan::ChildExited) => {
                return self.finish_windows_root_exit().map(Some);
            }
            Ok(WindowsProcessScan::MissingCommandLine { .. }) => {
                unreachable!("retry controller must resolve missing command-line metadata")
            }
            #[cfg(test)]
            Ok(WindowsProcessScan::MissingProcessIdentity { .. }) => {
                unreachable!("retry controller must resolve process identity metadata")
            }
            #[cfg(test)]
            Ok(WindowsProcessScan::UnverifiableAncestry { .. }) => {
                unreachable!("retry controller must reject ambiguous process ancestry")
            }
            Err(error) => Err(error),
        };

        match process_scan {
            Ok(Some(found)) => Err(self.fail_forbidden_process(&found, true)),
            Ok(None) => Ok(None),
            Err(error) => Err(self.fail_process_inspection(
                error.into(),
                "during command execution",
                "failing closed after process inspection failed",
                "failing closed after process inspection failed",
            )),
        }
    }
}

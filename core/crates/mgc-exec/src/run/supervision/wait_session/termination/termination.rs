//! Tree termination, child kill, and termination-failure aggregation.
//! Dừng cây, kill child và gộp lỗi trong lúc termination.

use super::super::implementation::WaitSession;
use crate::run::implementation::supervision::implementation::error::SupervisionError;
use anyhow::Result;

pub(in crate::run::implementation::supervision::implementation::wait_session) fn kill_child(
    child: &mut std::process::Child,
    action: &'static str,
) -> Result<()> {
    match child.kill() {
        Ok(()) => Ok(()),
        Err(source) if source.kind() == std::io::ErrorKind::InvalidInput => {
            match child.try_wait().map_err(SupervisionError::WaitChild)? {
                Some(_) => Ok(()),
                None => Err(SupervisionError::KillChild { action, source }.into()),
            }
        }
        Err(source) => Err(SupervisionError::KillChild { action, source }.into()),
    }
}

pub(in crate::run::implementation::supervision::implementation::wait_session) fn termination_failure(
    action: &'static str,
    termination: Option<anyhow::Error>,
    child_kill: Option<anyhow::Error>,
    trigger: anyhow::Error,
) -> anyhow::Error {
    #[cfg(any(unix, windows))]
    if termination.is_some() || child_kill.is_some() {
        return SupervisionError::TerminationFailure {
            action,
            trigger,
            termination,
            child_kill,
        }
        .into();
    }
    #[cfg(not(any(unix, windows)))]
    let _ = (action, termination, child_kill);
    trigger
}

impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn terminate_tree(
        &mut self,
    ) -> Option<anyhow::Error> {
        #[cfg(unix)]
        {
            crate::run::implementation::terminate_process_tree_tracked(
                self.child.id(),
                &mut self.process_system,
                &mut self.monitored_process_pids,
                &mut self.observed_process_groups,
                self.process_monitor.tracking_token,
                self.process_monitor.exempt,
                self.process_monitor.shadow_dir,
                self.root_reaped,
                self.process_monitor.execution_scope,
            )
            .err()
            .map(anyhow::Error::from)
        }
        #[cfg(windows)]
        {
            self.process_tree_guard
                .as_ref()
                .expect("Windows process-tree guard remains owned until root-exit completion")
                .terminate()
                .err()
                .map(anyhow::Error::from)
        }
        #[cfg(not(any(unix, windows)))]
        {
            None
        }
    }

    pub(in crate::run::implementation::supervision::implementation::wait_session) fn release_tree_guard(
        &mut self,
    ) {
        #[cfg(windows)]
        drop(self.process_tree_guard.take());
    }
}

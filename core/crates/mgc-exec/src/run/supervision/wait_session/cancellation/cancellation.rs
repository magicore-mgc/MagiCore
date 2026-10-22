//! Fail the wait session after a shutdown or explicit cancellation request.
//! Kết thúc phiên chờ khi có shutdown hoặc yêu cầu hủy tường minh.

use super::super::implementation::WaitSession;
use super::super::termination::{kill_child, termination_failure};
use crate::run::implementation::{POST_KILL_DRAIN_MS, shutdown_requested};
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

impl WaitSession<'_> {
    pub(in crate::run::implementation::supervision::implementation::wait_session) fn fail_if_cancelled(
        &mut self,
        cancel_flag: Option<&AtomicBool>,
    ) -> Result<()> {
        if !shutdown_requested() && !cancel_flag.is_some_and(|flag| flag.load(Ordering::SeqCst)) {
            return Ok(());
        }

        let termination_error = self.terminate_tree();
        self.release_tree_guard();
        let child_kill = kill_child(&mut self.child, "cancelling execution").err();
        let drained = self.output_capture.drain(
            &mut self.child,
            Some(Duration::from_millis(POST_KILL_DRAIN_MS)),
        );
        let trigger = match drained {
            Ok(_) if shutdown_requested() => anyhow::anyhow!(
                "interrupted by shutdown request (SIGINT/SIGTERM); process tree terminated"
            ),
            Ok(_) => {
                anyhow::anyhow!("execution cancelled by its supervisor; process tree terminated")
            }
            Err(error) => error,
        };
        Err(termination_failure(
            "cancelling execution",
            termination_error,
            child_kill,
            trigger,
        ))
    }
}

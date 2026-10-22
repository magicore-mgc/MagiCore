use std::panic;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::parallel::ParallelTask;
use crate::run::{ExecOptions, ExecReport};

/// Execute one task and update first-failure/cancellation state exactly once.
pub(in crate::parallel) fn run_task(
    index: usize,
    task: &ParallelTask,
    parallel_opts: &ExecOptions,
    cancellation: &AtomicBool,
    first_failure: &AtomicUsize,
    kill_others: bool,
) -> (usize, Result<ExecReport, String>) {
    let mut task_opts = parallel_opts.clone();
    task_opts.capture_full_stdout = true;
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        crate::run::run_cancellable_capture(&task.program, &task.args, &task_opts, cancellation)
    }))
    .map_err(|_| "parallel task executor panicked".to_string())
    .and_then(|result| result.map_err(|error| format!("{error:#}")));

    let failed = match &result {
        Ok(report) => {
            report.exit_code != 0 && !parallel_opts.allowed_exit_codes.contains(&report.exit_code)
        }
        Err(_) => true,
    };
    if failed {
        let _ =
            first_failure.compare_exchange(usize::MAX, index, Ordering::SeqCst, Ordering::SeqCst);
        if kill_others {
            cancellation.store(true, Ordering::SeqCst);
        }
    }
    (index, result)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

use anyhow::Result;

use crate::parallel::ParallelTask;
use crate::run::ExecOptions;

/// Reject the batch before any task starts if a direct invocation fails policy.
pub(crate) fn preflight_tasks(
    tasks: &[ParallelTask],
    original_opts: &ExecOptions,
    parallel_opts: &ExecOptions,
) -> Result<()> {
    for task in tasks {
        if !crate::run::compat_runner_declared(&task.program, original_opts) {
            let _ = crate::run::check_invocation_policy(&task.program, &task.args, parallel_opts)?;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

use anyhow::Result;

use super::{
    ParallelOptions, ParallelTask, execute_tasks, preflight_tasks, print_task_outputs,
    resolve_task_results,
};
use crate::run::ExecOptions;

/// Launch every task through the guarded executor and fail on the first failure.
/// Chạy mọi task qua executor có guard và fail khi task đầu tiên lỗi.
///
/// Every invocation is preflighted before any task starts, then checked again
/// immediately before its own spawn to close the scheduling window.
/// Mọi invocation được preflight trước khi chạy task nào, rồi kiểm tra lại
/// ngay trước spawn để đóng khoảng trễ do scheduler.
pub fn run_parallel(
    tasks: &[ParallelTask],
    opts: &ExecOptions,
    popts: &ParallelOptions,
) -> Result<Vec<super::ParallelResult>> {
    if tasks.is_empty() {
        return Ok(Vec::new());
    }

    let mut parallel_opts = (*opts).clone();
    parallel_opts
        .execution_scope
        .get_or_insert(crate::allowlist::ExecutionScope::BuildRunner);
    preflight_tasks(tasks, opts, &parallel_opts)?;

    let (task_results, failed_index) = execute_tasks(tasks, &parallel_opts, popts.kill_others);
    if popts.prefix_output {
        print_task_outputs(&task_results);
    }

    resolve_task_results(task_results, failed_index)
}

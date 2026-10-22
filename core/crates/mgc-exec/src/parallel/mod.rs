mod failure_report;
mod options;
mod parallel;
mod prefixed_output;
mod preflight;
mod result;
mod task;
mod task_threads;
mod task_worker;

pub use options::ParallelOptions;
pub use parallel::run_parallel;
pub use result::ParallelResult;
pub use task::ParallelTask;

pub(super) use failure_report::resolve_task_results;
pub(super) use prefixed_output::print_task_outputs;
pub(super) use preflight::preflight_tasks;
pub(super) use task_threads::execute_tasks;

#[cfg(test)]
pub(crate) mod test;

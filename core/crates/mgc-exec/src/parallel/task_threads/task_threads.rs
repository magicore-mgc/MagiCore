use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

use crate::parallel::ParallelTask;
use crate::run::{ExecOptions, ExecReport};

use super::super::task_worker::run_task;

pub(crate) type ParallelTaskResult = (usize, Result<ExecReport, String>);
pub(crate) type ParallelTaskSummary = (Vec<ParallelTaskResult>, usize);

/// Run scoped workers, join them in input order, and retain the first failure index.
pub(crate) fn execute_tasks(
    tasks: &[ParallelTask],
    parallel_opts: &ExecOptions,
    kill_others: bool,
) -> ParallelTaskSummary {
    let cancellation = AtomicBool::new(false);
    let first_failure = AtomicUsize::new(usize::MAX);
    let task_results = thread::scope(|threads| {
        let mut handles = Vec::with_capacity(tasks.len());
        for (index, task) in tasks.iter().enumerate() {
            let cancellation = &cancellation;
            let first_failure = &first_failure;
            handles.push(threads.spawn(move || {
                run_task(
                    index,
                    task,
                    parallel_opts,
                    cancellation,
                    first_failure,
                    kill_others,
                )
            }));
        }

        handles
            .into_iter()
            .enumerate()
            .map(|(index, handle)| match handle.join() {
                Ok(result) => result,
                Err(_) => {
                    let _ = first_failure.compare_exchange(
                        usize::MAX,
                        index,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    );
                    if kill_others {
                        cancellation.store(true, Ordering::SeqCst);
                    }
                    (index, Err("parallel task worker panicked".to_string()))
                }
            })
            .collect::<Vec<_>>()
    });
    (task_results, first_failure.load(Ordering::SeqCst))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

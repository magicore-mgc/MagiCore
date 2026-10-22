use std::sync::atomic::{AtomicBool, AtomicUsize};

use super::super::run_task;
use crate::parallel::test::support::task;
use crate::run::ExecOptions;

#[test]
fn task_failure_sets_the_first_failure_index() {
    let cancellation = AtomicBool::new(false);
    let first_failure = AtomicUsize::new(usize::MAX);
    let (index, result) = run_task(
        4,
        &task("python3", &["-c", "import sys; sys.exit(3)"]),
        &ExecOptions::default(),
        &cancellation,
        &first_failure,
        false,
    );
    assert_eq!(index, 4);
    assert!(result.is_ok());
    assert_eq!(first_failure.load(std::sync::atomic::Ordering::SeqCst), 4);
}

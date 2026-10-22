use super::super::execute_tasks;
use crate::run::ExecOptions;

#[test]
fn empty_scoped_batch_returns_no_workers_or_failures() {
    let (results, failure) = execute_tasks(&[], &ExecOptions::default(), true);
    assert!(results.is_empty());
    assert_eq!(failure, usize::MAX);
}

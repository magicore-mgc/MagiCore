use super::super::preflight_tasks;
use crate::parallel::test::support::task;
use crate::run::ExecOptions;

#[test]
fn rejects_forbidden_direct_invocation_before_workers_start() {
    let options = ExecOptions::default();
    assert!(preflight_tasks(&[task("npm", &["install", "x"])], &options, &options).is_err());
}

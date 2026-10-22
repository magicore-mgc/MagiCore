use super::super::resolve_task_results;

#[test]
fn reports_the_recorded_worker_error_with_its_task_index() {
    let error = resolve_task_results(vec![(2, Err("worker stopped".to_string()))], 2)
        .expect_err("recorded failure must remain an error");
    assert!(
        error
            .to_string()
            .contains("parallel task 2 failed: worker stopped")
    );
}

#[test]
fn rejects_a_failure_index_missing_from_the_joined_results() {
    let error = resolve_task_results(Vec::new(), 7)
        .expect_err("missing task result must remain a supervisor error");
    assert!(
        error
            .to_string()
            .contains("parallel task supervisor lost the first failing task")
    );
}

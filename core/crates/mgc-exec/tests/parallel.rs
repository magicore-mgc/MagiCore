use mgc_exec::parallel::{ParallelOptions, ParallelTask, run_parallel};
#[test]
fn parallel_public_api_accepts_an_empty_batch() {
    let task = ParallelTask {
        program: "python3".to_string(),
        args: vec!["-c".to_string(), "pass".to_string()],
    };
    assert_eq!(task.program, "python3");
    let result = run_parallel(
        &[],
        &mgc_exec::run::ExecOptions::default(),
        &ParallelOptions::default(),
    );
    assert!(matches!(result, Ok(results) if results.is_empty()));
}

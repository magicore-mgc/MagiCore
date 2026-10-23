use super::super::ParallelResult;

#[test]
fn retains_index_exit_code_and_captured_streams() {
    let result = ParallelResult {
        index: 3,
        exit_code: 1,
        stdout: "out".to_string(),
        stderr: "err".to_string(),
    };
    assert_eq!(result.index, 3);
    assert_eq!(result.exit_code, 1);
    assert_eq!(result.stdout, "out");
    assert_eq!(result.stderr, "err");
}

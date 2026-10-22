// Tests for the shared output mode contract.
// Test hợp đồng chế độ output dùng chung.

use super::*;

#[test]
fn output_mode_is_copyable_and_keeps_capture_distinct_from_inherit() {
    let capture = OutputMode::Capture;
    let copied_capture = capture;
    assert_eq!(capture, copied_capture);
    assert_ne!(copied_capture, OutputMode::Inherit);
}

#[test]
fn execution_outcome_keeps_exit_status_and_stream_bytes_together() {
    let outcome = ExecOutcome {
        status: successful_exit_status(),
        stdout: b"captured stdout".to_vec(),
        stderr: b"captured stderr".to_vec(),
    };
    assert_eq!(outcome.status.code(), Some(0));
    assert_eq!(outcome.stdout, b"captured stdout");
    assert_eq!(outcome.stderr, b"captured stderr");
}

#[cfg(unix)]
fn successful_exit_status() -> ExitStatus {
    use std::os::unix::process::ExitStatusExt;
    ExitStatus::from_raw(0)
}

#[cfg(windows)]
fn successful_exit_status() -> ExitStatus {
    use std::os::windows::process::ExitStatusExt;
    ExitStatus::from_raw(0)
}

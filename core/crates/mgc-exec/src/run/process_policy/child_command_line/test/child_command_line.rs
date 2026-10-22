// Focused tests for empty and present child command lines.
// Test tập trung cho command line child rỗng và có dữ liệu.

use super::*;
use std::ffi::OsString;

#[test]
fn rejects_empty_and_accepts_present_command_line() {
    let error = validate_monitored_child_command_line("worker.exe", &[], 17)
        .expect_err("empty child command lines are unavailable");
    assert!(matches!(
        error,
        ProcessPolicyError::MissingChildCommandLine {
            image_name,
            process_id: 17
        } if image_name == "worker.exe"
    ));

    assert!(
        validate_monitored_child_command_line("worker.exe", &[OsString::from("worker")], 17).is_ok()
    );
}

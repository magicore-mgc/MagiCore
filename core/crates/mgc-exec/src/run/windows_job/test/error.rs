//! Source-chain tests for the Windows Job Object error boundary.
//! Test chuỗi nguồn cho ranh giới lỗi Windows Job Object.

use super::{WindowsApiFailure, WindowsJobError};

#[test]
fn native_api_failure_retains_operation_and_code_as_source() {
    let error = WindowsJobError::native_api("AssignProcessToJobObject", 5);
    let display = error.to_string();
    assert!(display.contains("AssignProcessToJobObject"));
    assert!(display.contains("5"));

    let source =
        std::error::Error::source(&error).expect("native API failure must retain its typed source");
    assert!(source.to_string().contains("AssignProcessToJobObject"));
    assert!(source.to_string().contains("5"));
}

#[test]
fn membership_failure_keeps_process_identity_and_native_cause() {
    let error = WindowsJobError::MembershipQueryFailed {
        process_id: 2718,
        source: WindowsApiFailure::new("IsProcessInJob", 5),
    };

    assert!(error.to_string().contains("2718"));
    assert!(
        std::error::Error::source(&error)
            .expect("membership query failure must retain its API source")
            .to_string()
            .contains("IsProcessInJob")
    );
}

#[test]
fn handle_close_aggregation_keeps_native_causes_and_bounds_detail_output() {
    let error = WindowsJobError::handle_close_failures(
        vec![
            WindowsApiFailure::new("CloseHandle", 6),
            WindowsApiFailure::new("CloseHandle", 87),
        ],
        3,
    );

    let message = error.to_string();
    assert!(message.contains("5 Windows handle(s)"));
    assert!(message.contains("error code 6"));
    assert!(message.contains("error code 87"));
    assert!(message.contains("3 additional failure(s) omitted"));
    assert_eq!(
        std::error::Error::source(&error)
            .expect("at least the first close failure must remain the source")
            .to_string(),
        "Windows API CloseHandle failed with error code 6"
    );
}

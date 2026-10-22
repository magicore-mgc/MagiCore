//! Tests that membership mismatch is rejected for a live external process.
//! Kiểm tra membership mismatch bị từ chối với process ngoài còn sống.

use super::super::super::job_lifecycle::ProcessTreeGuard;
use super::super::super::super::error::WindowsJobError;

#[test]
fn member_capture_rejects_process_outside_private_job() {
    let guard = ProcessTreeGuard::new().expect("private Job Object should be created");
    let process_id = std::process::id();
    let result = guard.capture_job_member_process(process_id);
    assert!(matches!(
        result,
        Err(WindowsJobError::MembershipChanged { process_id: actual }) if actual == process_id
    ));
}

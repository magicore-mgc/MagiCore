//! Tests the native PID query against an empty private Job Object.
//! Kiểm tra truy vấn PID native với Job Object riêng đang rỗng.

#[test]
fn empty_job_process_snapshot_is_empty() {
    let guard = super::super::super::job_lifecycle::ProcessTreeGuard::new()
        .expect("private Job Object should be created");
    assert!(guard
        .active_process_ids()
        .expect("empty Job PID query should succeed")
        .is_empty());
}

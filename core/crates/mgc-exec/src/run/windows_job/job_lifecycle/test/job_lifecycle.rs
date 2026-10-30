// Tests creation and explicit termination of an empty private Job Object.
// Kiểm tra tạo và terminate Job Object riêng đang rỗng.

#[test]
fn empty_job_can_be_terminated() {
    let guard = super::ProcessTreeGuard::new()
        .expect("private Job Object should be created");
    guard.terminate().expect("empty Job Object should terminate");
}

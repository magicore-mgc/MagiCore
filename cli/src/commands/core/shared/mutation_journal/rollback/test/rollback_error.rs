//! Mutation rollback error-preservation contract.
//! Contract giữ lại lỗi mutation và lỗi rollback.

use super::super::combine_rollback_errors;

#[test]
fn rollback_error_carries_both_install_and_restore_failures() {
    // P0: restore failure must propagate INSIDE the returned error —
    // warning-only would hide a half-updated project from CI/API.
    // (Lỗi restore phải nằm trong error trả về — warning là nuốt lỗi.)
    let err = combine_rollback_errors("fetch broke: connection refused", "disk read-only");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("fetch broke: connection refused"),
        "install error must surface: {msg}"
    );
    assert!(
        msg.contains("disk read-only"),
        "restore error must surface: {msg}"
    );
    assert!(
        msg.contains("half-updated"),
        "must flag possible half-updated state: {msg}"
    );
}

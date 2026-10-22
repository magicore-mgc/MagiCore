//! Tests that Windows PATH integration is scoped to the current user.
//! Kiểm tra tích hợp PATH chỉ áp dụng cho user hiện tại.

use super::super::USER_PATH_SCRIPT;

#[test]
fn script_reads_and_updates_user_path_without_machine_scope() {
    assert!(USER_PATH_SCRIPT.contains("GetEnvironmentVariable('Path', 'User')"));
    assert!(USER_PATH_SCRIPT.contains("SetEnvironmentVariable('Path'"));
    assert!(USER_PATH_SCRIPT.contains("'User'"));
    assert!(!USER_PATH_SCRIPT.contains("'Machine'"));
}

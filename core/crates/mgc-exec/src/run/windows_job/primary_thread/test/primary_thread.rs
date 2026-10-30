// Tests the primary-thread suspend-count acceptance boundary.
// Kiểm tra ranh giới chấp nhận suspend count của primary thread.

use super::validate_primary_suspend_count;

#[test]
fn primary_thread_resume_requires_single_suspend() {
    assert!(validate_primary_suspend_count(42, 1).is_ok());
    assert!(validate_primary_suspend_count(42, 0).is_err());
    assert!(validate_primary_suspend_count(42, 2).is_err());
}

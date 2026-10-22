//! Tests for Swift canonical-lock publication eligibility.
//! Test điều kiện công bố canonical lock cho Swift.

use super::super::should_write_swift_lock;

#[test]
fn swift_lock_is_skipped_only_for_replay() {
    assert!(should_write_swift_lock(false));
    assert!(!should_write_swift_lock(true));
}

//! Tests for Flutter canonical-lock publication eligibility.
//! Test điều kiện công bố canonical lock cho Flutter.

use super::super::should_write_flutter_lock;

#[test]
fn flutter_lock_requires_online_non_replay_install() {
    assert!(should_write_flutter_lock(false, false));
    assert!(!should_write_flutter_lock(true, false));
    assert!(!should_write_flutter_lock(false, true));
    assert!(!should_write_flutter_lock(true, true));
}

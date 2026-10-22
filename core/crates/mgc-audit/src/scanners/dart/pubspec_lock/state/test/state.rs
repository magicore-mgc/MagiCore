//! Pub lock state lifecycle tests.
//! Test vòng đời state parser lockfile Pub.

use super::*;

#[test]
fn default_state_is_outside_packages_and_finishes_empty() {
    let state = PubspecLockState::default();
    assert!(!state.in_packages);
    assert!(state.current_name.is_none());
    let (pins, skipped) = state.finish();
    assert!(pins.is_empty());
    assert!(skipped.is_empty());
}

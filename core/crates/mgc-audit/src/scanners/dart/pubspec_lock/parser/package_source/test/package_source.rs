//! Package source skip tests.
//! Test bỏ qua theo nguồn package.

use super::*;

#[test]
fn hosted_source_does_not_add_a_skip() {
    let mut state = PubspecLockState {
        current_name: Some("http".to_string()),
        ..PubspecLockState::default()
    };
    record_if_unsupported(&mut state, "source: hosted");
    assert!(state.skipped.is_empty());
}

#[test]
fn explicit_non_hosted_source_keeps_exact_reason() {
    let mut state = PubspecLockState {
        current_name: Some("local".to_string()),
        ..PubspecLockState::default()
    };
    record_if_unsupported(&mut state, "source: path");
    assert_eq!(state.skipped.len(), 1);
    assert_eq!(
        state.skipped[0],
        "local: source 'path' has no advisory mapping"
    );
}

#[test]
fn empty_source_and_missing_package_are_ignored() {
    let mut state = PubspecLockState::default();
    record_if_unsupported(&mut state, "source:");
    assert!(state.skipped.is_empty());
}

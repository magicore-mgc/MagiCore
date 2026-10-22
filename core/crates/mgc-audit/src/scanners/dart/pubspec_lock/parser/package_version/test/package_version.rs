//! Package version projection tests.
//! Test ánh xạ version package.

use super::*;

#[test]
fn quoted_version_is_trimmed_into_a_pub_pin() {
    let mut state = PubspecLockState {
        current_name: Some("http".to_string()),
        ..PubspecLockState::default()
    };
    record_if_version(&mut state, "version: \"1.2.2\"");
    assert_eq!(state.pins[0].name, "http");
    assert_eq!(state.pins[0].version, "1.2.2");
    assert_eq!(state.pins[0].ecosystem, "Pub");
}

#[test]
fn empty_version_keeps_the_existing_skip_message() {
    let mut state = PubspecLockState {
        current_name: Some("http".to_string()),
        ..PubspecLockState::default()
    };
    record_if_version(&mut state, "version:");
    assert_eq!(state.skipped.len(), 1);
    assert_eq!(state.skipped[0], "http: empty version pin");
}

#[test]
fn version_without_package_is_ignored() {
    let mut state = PubspecLockState::default();
    record_if_version(&mut state, "version: 1.0");
    assert!(state.pins.is_empty());
    assert!(state.skipped.is_empty());
}

//! Package block recognition tests.
//! Test nhận diện block package.

use super::*;

#[test]
fn package_name_header_replaces_current_name() {
    let mut state = PubspecLockState::default();
    assert!(begin_if_header(&mut state, "async:"));
    assert_eq!(state.current_name.as_deref(), Some("async"));
}

#[test]
fn known_fields_are_not_package_headers() {
    for field in [
        "version: 1:",
        "source: hosted:",
        "description: value:",
        "sha256: value:",
        "url: value:",
        "source-map: value:",
    ] {
        assert!(!begin_if_header(&mut PubspecLockState::default(), field));
    }
}

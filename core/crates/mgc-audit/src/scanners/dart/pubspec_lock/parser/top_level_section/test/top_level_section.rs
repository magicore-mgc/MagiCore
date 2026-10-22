//! Pub lock top-level section tests.
//! Test section cấp gốc lockfile Pub.

use super::*;

#[test]
fn packages_key_activates_section_and_resets_package_name() {
    let mut state = PubspecLockState {
        in_packages: false,
        current_name: Some("old".to_string()),
        ..PubspecLockState::default()
    };
    reset_for_root_key(&mut state, "packages:");
    assert!(state.in_packages);
    assert!(state.current_name.is_none());
}

#[test]
fn any_other_root_key_disables_package_parsing() {
    let mut state = PubspecLockState {
        in_packages: true,
        current_name: Some("old".to_string()),
        ..PubspecLockState::default()
    };
    reset_for_root_key(&mut state, "sdks:");
    assert!(!state.in_packages);
    assert!(state.current_name.is_none());
}

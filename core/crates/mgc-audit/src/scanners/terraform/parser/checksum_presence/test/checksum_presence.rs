//! Test Terraform checksum-array presence recognition.
//! Kiểm tra nhận diện checksum-array Terraform.
#![allow(clippy::unwrap_used)]

use super::observe;
use crate::scanners::terraform::types::TerraformLockState;

#[test]
fn hashes_and_legacy_checksums_openers_mark_presence() {
    let mut state = TerraformLockState::default();
    observe("hashes = [", &mut state);
    assert!(state.current_has_checksum);

    state.current_has_checksum = false;
    observe("checksums = [", &mut state);
    assert!(state.current_has_checksum);
}

#[test]
fn absent_bracket_or_unrecognized_key_does_not_mark_presence() {
    let mut state = TerraformLockState::default();
    observe("hashes =", &mut state);
    observe("other = [", &mut state);

    assert!(!state.current_has_checksum);
}

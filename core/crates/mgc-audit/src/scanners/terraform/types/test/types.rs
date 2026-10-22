//! Test default Terraform parser state.
//! Kiểm tra parser state Terraform mặc định.

use super::TerraformLockState;

#[test]
fn default_state_has_no_provider_or_failures() {
    let state = TerraformLockState::default();

    assert_eq!(state.providers, 0);
    assert!(state.current_provider.is_none());
    assert!(!state.current_has_checksum);
    assert!(state.failures.is_empty());
}

//! Test Terraform provider state transitions.
//! Kiểm tra chuyển trạng thái provider Terraform.
#![allow(clippy::unwrap_used)]

use super::{close_if_block_end, finish_at_eof, start_if_header};
use crate::scanners::terraform::error::TerraformLockError;
use crate::scanners::terraform::types::TerraformLockState;

#[test]
fn provider_headers_count_and_transition_in_order() {
    let mut state = TerraformLockState::default();
    assert!(
        start_if_header(
            "provider \"registry.terraform.io/hashicorp/aws\" {",
            &mut state
        )
        .unwrap()
    );
    assert!(
        start_if_header(
            "provider \"registry.terraform.io/hashicorp/null\" {",
            &mut state
        )
        .unwrap()
    );
    state.current_has_checksum = true;
    finish_at_eof(&mut state);

    assert_eq!(state.providers, 2);
    assert_eq!(
        state.failures,
        vec!["provider 'registry.terraform.io/hashicorp/aws' has no checksums".to_string()]
    );
}

#[test]
fn closing_provider_block_records_missing_checksum() {
    let mut state = TerraformLockState::default();
    start_if_header("provider \"registry.example/test\" {", &mut state).unwrap();
    close_if_block_end("}", &mut state);

    assert_eq!(
        state.failures,
        vec!["provider 'registry.example/test' has no checksums".to_string()]
    );
    assert!(state.current_provider.is_none());
}

#[test]
fn malformed_header_keeps_the_original_text() {
    let mut state = TerraformLockState::default();
    let error = start_if_header("provider \"broken", &mut state).unwrap_err();

    assert_eq!(
        error,
        TerraformLockError::MalformedProviderHeader {
            header: "provider \"broken".to_string()
        }
    );
}

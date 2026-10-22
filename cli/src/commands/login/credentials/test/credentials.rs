//! Test password input normalization and boundaries.
//! Kiểm tra chuẩn hóa và giới hạn password input.
#![allow(clippy::unwrap_used)]

use super::super::CredentialInputError;
use super::{MAX_PASSWORD_STDIN_BYTES, parse_password_stdin, require_non_empty};

#[test]
fn stdin_password_strips_only_line_endings() {
    assert_eq!(
        parse_password_stdin(" secret \r\n".to_owned()).unwrap(),
        " secret "
    );
}

#[test]
fn stdin_password_rejects_byte_limit_overflow() {
    let oversized = "x".repeat(MAX_PASSWORD_STDIN_BYTES + 1);
    assert!(matches!(
        parse_password_stdin(oversized),
        Err(CredentialInputError::TooLarge)
    ));
}

#[test]
fn empty_password_is_rejected_for_both_input_modes() {
    assert!(matches!(
        parse_password_stdin("\r\n".to_owned()),
        Err(CredentialInputError::Empty)
    ));
    assert!(matches!(
        require_non_empty(String::new()),
        Err(CredentialInputError::Empty)
    ));
}

#[test]
fn prompt_errors_keep_the_original_context_chain() {
    use anyhow::Context;

    let original = anyhow::anyhow!("terminal failed").context("password prompt failed");
    let expected = format!("{original:#}");
    let actual = super::super::credential_input_diagnostic(CredentialInputError::Prompt(original));
    assert_eq!(format!("{actual:#}"), expected);
}

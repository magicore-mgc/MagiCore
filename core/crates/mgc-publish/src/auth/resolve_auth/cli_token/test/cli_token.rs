//! CLI token source tests / test nguồn token CLI.

use super::super::resolve_cli_token;

#[test]
fn present_cli_token_is_selected() {
    assert_eq!(
        resolve_cli_token(Some("cli-token")).and_then(|auth| auth.token),
        Some("cli-token".into())
    );
}

#[test]
fn absent_cli_token_has_no_result() {
    assert!(resolve_cli_token(None).is_none());
}

//! Token normalization cases — ca kiểm thử chuẩn hóa token.
//!
use super::super::normalize_token;

#[test]
fn trims_nonempty_token_without_changing_its_contents() {
    assert_eq!(
        normalize_token("  jwt-value\n".to_owned()).as_deref(),
        Some("jwt-value")
    );
}

#[test]
fn treats_empty_or_whitespace_token_as_absent() {
    assert_eq!(normalize_token(String::new()), None);
    assert_eq!(normalize_token(" \t\n".to_owned()), None);
}

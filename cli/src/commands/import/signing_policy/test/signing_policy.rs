//! Import signing decision cases — ca quyết định ký import.
#![allow(clippy::unwrap_used)]

use super::super::{ImportSigningMode, decide_signing_mode};
use crate::commands::import::ImportError;

#[test]
fn an_available_default_key_requires_signed_mode() {
    assert_eq!(
        decide_signing_mode(true, false).unwrap(),
        ImportSigningMode::Signed
    );
    assert_eq!(
        decide_signing_mode(true, true).unwrap(),
        ImportSigningMode::Signed
    );
}

#[test]
fn absent_key_allows_unsigned_only_after_explicit_opt_in() {
    assert_eq!(
        decide_signing_mode(false, true).unwrap(),
        ImportSigningMode::UnsignedExplicit
    );
}

#[test]
fn absent_key_without_opt_in_keeps_the_existing_refusal() {
    let error = decide_signing_mode(false, false).unwrap_err();
    assert!(matches!(error, ImportError::UnsignedOptInRequired));
    assert_eq!(
        error.to_string(),
        "import refused: no default signing key and --allow-unsigned not given (unsigned lockfiles are never the default) — generate a key (`mgc trust`) or re-run with --allow-unsigned"
    );
}

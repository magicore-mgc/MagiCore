//! Tests for the Lib offline install allowlist.
//! Test allowlist install offline của Lib.

use super::super::validate_offline_install;
use crate::language::LibLanguage;

#[test]
fn offline_allowlist_is_exactly_ts_python_and_rust() {
    for language in [LibLanguage::Ts, LibLanguage::Python, LibLanguage::Rust] {
        assert!(validate_offline_install(language, true).is_ok());
    }
    for language in [LibLanguage::Go, LibLanguage::Java, LibLanguage::DotNet] {
        let error = validate_offline_install(language, true).unwrap_err();
        assert!(error.to_string().contains("offline install is unsupported"));
    }
}

#[test]
fn online_install_passes_the_offline_gate_for_every_language() {
    for language in [
        LibLanguage::Ts,
        LibLanguage::Rust,
        LibLanguage::Python,
        LibLanguage::Go,
        LibLanguage::Java,
        LibLanguage::DotNet,
    ] {
        assert!(validate_offline_install(language, false).is_ok());
    }
}

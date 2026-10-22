// Package spec splitter tests — test bộ tách package spec.
// Cover scoped identifiers and invalid/missing versions.
// Bao phủ identifier có scope và version sai hoặc bị thiếu.

use super::split_name_version;

#[test]
fn splits_scoped_names_at_the_final_at_sign() {
    assert_eq!(
        split_name_version("@scope/pkg@1.2.3"),
        Some(("@scope/pkg".to_string(), "1.2.3".to_string()))
    );
}

#[test]
fn rejects_missing_name_or_version() {
    for pin in ["", "@scope/pkg", "pkg@"] {
        assert_eq!(split_name_version(pin), None);
    }
}

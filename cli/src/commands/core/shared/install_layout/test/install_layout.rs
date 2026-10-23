//! Web install-layout parsing contract tests.
//! Test contract parse layout install Web.

use super::super::install_layout::legacy_flat_layout_for;

#[test]
fn legacy_values_select_the_flat_layout_after_normalization() {
    for value in ["0", "false", "no", "off", "legacy", "flat", " FLAT "] {
        assert!(legacy_flat_layout_for("web", Some(value)), "value: {value}");
    }
}

#[test]
fn strict_unknown_and_missing_values_select_the_strict_layout() {
    for value in ["1", "true", "yes", "on", "unknown", ""] {
        assert!(
            !legacy_flat_layout_for("web", Some(value)),
            "value: {value}"
        );
    }
    assert!(!legacy_flat_layout_for("web", None));
}

#[test]
fn non_web_cores_never_select_the_web_legacy_layout() {
    for core_type in [
        "ai", "app", "lib", "game", "cloud", "iot", "cicd", "hardware", "unknown",
    ] {
        assert!(
            !legacy_flat_layout_for(core_type, Some("legacy")),
            "core: {core_type}"
        );
    }
}

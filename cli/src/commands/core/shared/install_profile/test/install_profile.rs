//! Contract tests for accepted install profiling values.
//! Test contract cho các giá trị bật profiling install.

use super::super::implementation::profile_enabled;

#[test]
fn profile_parser_preserves_true_values_case_and_whitespace() {
    for value in ["1", "true", "TRUE", " Yes ", "on"] {
        assert!(
            profile_enabled(Some(value)),
            "expected enabled for {value:?}"
        );
    }
}

#[test]
fn profile_parser_disables_missing_false_and_unknown_values() {
    for value in [
        None,
        Some(""),
        Some("0"),
        Some("false"),
        Some("off"),
        Some("debug"),
    ] {
        assert!(!profile_enabled(value), "expected disabled for {value:?}");
    }
}

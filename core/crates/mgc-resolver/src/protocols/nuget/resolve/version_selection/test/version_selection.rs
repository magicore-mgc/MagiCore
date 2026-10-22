//! Behavior tests for the version_selection owner.
//! Test hành vi cho owner version_selection.

use super::super::*;

#[test]
fn lowest_matching_listed_version_skips_malformed_values() {
    let versions = ["bad-version", "2.0.0", "1.5.0", "1.2.0"];

    assert_eq!(
        lowest_matching_version(&versions, "[1.0,2.0)"),
        Some("1.2.0".into())
    );
}

#[test]
fn lowest_matching_version_preserves_first_raw_spelling_on_equal_precedence() {
    let versions = ["1.2", "1.2.0"];

    assert_eq!(lowest_matching_version(&versions, "*"), Some("1.2".into()));
}

#[test]
fn lowest_matching_version_returns_none_when_candidates_do_not_match() {
    let versions = ["2.0.0", "3.0.0"];

    assert_eq!(lowest_matching_version(&versions, "[1.0,2.0)"), None);
}

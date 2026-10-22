//! Tests the private helper directly from its owner subtree.
//! Kiểm tra helper private trực tiếp trong subtree owner.

use super::super::compare_prerelease::compare_prerelease;
use std::cmp::Ordering;

#[test]
fn release_sorts_after_prerelease() {
    assert_eq!(compare_prerelease(None, Some("rc.1")), Ordering::Greater);
}

#[test]
fn numeric_identifiers_compare_numerically() {
    assert_eq!(
        compare_prerelease(Some("rc.9"), Some("rc.24")),
        Ordering::Less
    );
}

#[test]
fn numeric_identifiers_sort_before_text() {
    assert_eq!(compare_prerelease(Some("1"), Some("alpha")), Ordering::Less);
}

#[test]
fn text_and_segment_count_follow_semver_order() {
    assert_eq!(
        compare_prerelease(Some("alpha"), Some("beta")),
        Ordering::Less
    );
    assert_eq!(
        compare_prerelease(Some("alpha"), Some("alpha.1")),
        Ordering::Less
    );
}

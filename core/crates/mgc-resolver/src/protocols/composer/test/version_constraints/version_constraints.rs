//! Composer constraint semantics tests.
//! Test ngữ nghĩa ràng buộc Composer.

use crate::protocols::composer::version_constraints::{composer_matches, strip_stability_flags};
use mgc_types::Version;

fn matches(range: &str, version: &str) -> bool {
    let parsed = Version::parse(version).expect("test version must parse");
    composer_matches(range, &parsed)
}

#[test]
fn caret_follows_composer_leftmost_non_zero_rule() {
    assert!(matches("^3.0", "3.12.1"));
    assert!(!matches("^2.0", "3.12.1"));
    assert!(matches("^0.3", "0.3.9"));
    assert!(!matches("^0.3", "0.4.0"));
    assert!(matches("^0.0.3", "0.0.3"));
    assert!(!matches("^0.0.3", "0.0.4"));
}

#[test]
fn tilde_allows_patch_and_minor_only() {
    assert!(matches("~1.2", "1.9.0"));
    assert!(!matches("~1.2", "2.0.0"));
    assert!(matches("~1.2.3", "1.2.9"));
    assert!(!matches("~1.2.3", "1.3.0"));
}

#[test]
fn unions_and_conjunctions_match() {
    assert!(matches("^2.0 || ^3.0", "3.12.1"));
    assert!(matches("^2.0|^3.0", "3.12.1"));
    assert!(!matches("^2.0 || ^3.0", "4.0.0"));
    assert!(matches(">=7.0 <8.0", "7.4.5"));
    assert!(!matches(">=7.0 <8.0", "8.0.0"));
    assert!(matches(">= 7.0, < 8.0", "7.4.5"));
    assert!(!matches(">= 7.0, < 8.0", "6.4.0"));
}

#[test]
fn wildcard_and_star_and_stability_flags() {
    assert!(matches("*", "3.12.1"));
    assert!(matches("", "3.12.1"));
    assert!(matches("1.*", "1.0.0"));
    assert!(!matches("1.*", "2.0.0"));
    assert!(matches("3.12.*", "3.12.1"));
    assert!(!matches("3.12.*", "3.13.0"));
    assert!(matches("~1.0@dev", "1.4.0"));
    assert!(matches("^3.0@stable", "3.12.1"));
    assert_eq!(strip_stability_flags("^2.0 || ^3.0"), "^2.0||^3.0");
    assert_eq!(strip_stability_flags("1.0@dev || 2.0@beta"), "1.0||2.0");
}

#[test]
fn exact_and_inequality_use_zero_padding() {
    assert!(matches("1.0.0", "1.0.0"));
    assert!(!matches("1.0.0", "1.0.1"));
    assert!(matches("!=1.0.0", "1.0.1"));
    assert!(!matches("!=1.0.0", "1.0.0"));
    assert!(matches(">=8.1", "8.1.0"));
    assert!(!matches(">=8.2", "8.1.0"));
    assert!(!matches("3.12.1", "3.12.1-RC1"));
}

#[test]
fn prerelease_versions_satisfy_their_own_range() {
    // `3.0.0-RC1` is parsed by the shared type as pre `RC1`; the Composer
    // matcher reads it as the same triple with a `rc1` tag.
    // `3.0.0-RC1` được kiểu dùng chung đọc là pre `RC1`; matcher Composer
    // đọc cùng bộ ba với tag `rc1`.
    assert!(!matches(">=3.0", "3.0.0-RC1"));
    assert!(matches("^3.0.0-RC1", "3.0.0-RC1"));
    assert!(matches("3.0.0-RC1", "3.0.0-RC1"));
    assert!(matches("~3.0.0-RC1", "3.0.0-RC1"));
    assert!(!matches("^3.0.0", "3.0.0-RC1"));
}

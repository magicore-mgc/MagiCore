//! Dart caret, exact, wildcard and range constraint tests.
//! Test constraint caret, exact, wildcard và range của Dart.

use super::super::dart_matches;
use mgc_types::Version;

#[test]
fn dart_caret_and_exact() {
    assert!(dart_matches(
        "^1.2.0",
        &Version::parse("1.9.0").expect("test operation must succeed")
    ));
    assert!(!dart_matches(
        "^1.2.0",
        &Version::parse("2.0.0").expect("test operation must succeed")
    ));
    assert!(dart_matches(
        "1.2.0",
        &Version::parse("1.2.0").expect("test operation must succeed")
    ));
    assert!(!dart_matches(
        "1.2.0",
        &Version::parse("1.2.1").expect("test operation must succeed")
    ));
}

#[test]
fn dart_any_and_wildcard() {
    assert!(dart_matches(
        "any",
        &Version::parse("9.9.9").expect("test operation must succeed")
    ));
    assert!(dart_matches(
        "1.2.*",
        &Version::parse("1.2.7").expect("test operation must succeed")
    ));
    assert!(!dart_matches(
        "1.2.*",
        &Version::parse("1.3.0").expect("test operation must succeed")
    ));
    assert!(dart_matches(
        ">=1.0.0 <2.0.0",
        &Version::parse("1.5.0").expect("test operation must succeed")
    ));
}

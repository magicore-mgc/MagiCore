//! Gradle metadata parser edge tests.
//! Test biên cho parser metadata Gradle.

use super::*;

#[test]
fn missing_coordinate_line_is_bounded_and_reported() {
    let raw = format!(
        "<dependency group=\"example\" name=\"missing-version\">{}</dependency>",
        "x".repeat(200)
    );
    let (pins, skipped) = read_gradle_verification_metadata(&raw);
    assert!(pins.is_empty());
    assert_eq!(skipped.len(), 1);
    assert!(skipped[0].contains("missing group/name/version"));
    assert!(skipped[0].len() < 150);
}

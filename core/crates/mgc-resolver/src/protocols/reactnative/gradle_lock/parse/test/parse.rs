//! Gradle lock parser behavior tests.
//! Test hành vi parser Gradle lock.

use super::*;

#[test]
fn skips_comments_and_preserves_pinned_fields() {
    let text = "# generated\ncom.example:core:1.2.0:compileClasspath\n";
    let entries = parse_entries(text).expect("test operation must succeed");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].group, "com.example");
    assert_eq!(entries[0].artifact, "core");
    assert_eq!(entries[0].version, "1.2.0");
    assert_eq!(entries[0].configuration, "compileClasspath");
}

#[test]
fn malformed_line_fails_closed_with_line_number() {
    let error = parse_entries("# generated\nonly:two\n").expect_err("test operation must fail");
    assert!(error.to_string().contains("gradle.lockfile line 2"));
}

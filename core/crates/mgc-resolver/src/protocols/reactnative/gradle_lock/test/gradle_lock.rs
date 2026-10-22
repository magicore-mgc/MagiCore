//! Gradle public-entry compatibility tests.
//! Test tương thích entry point công khai của Gradle.

use super::*;

#[test]
fn gradle_lockfile_parse_and_collapse() {
    let text = "# This is a Gradle generated file for dependency locking.\n\
                # Manual edits can break the build.\n\
                com.google.android.material:material:1.10.0:releaseCompileClasspath\n\
                com.google.android.material:material:1.10.0:debugCompileClasspath\n\
                androidx.core:core-ktx:1.12.0:releaseCompileClasspath\n";
    let entries = parse_gradle_lockfile(text).expect("test operation must succeed");
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].group, "com.google.android.material");
    assert_eq!(entries[0].artifact, "material");
    assert_eq!(entries[0].configuration, "releaseCompileClasspath");
    let pins = collapse_gradle_pins(&entries).expect("test operation must succeed");
    assert_eq!(
        pins,
        vec![
            (
                "com.google.android.material:material".to_string(),
                "1.10.0".to_string()
            ),
            ("androidx.core:core-ktx".to_string(), "1.12.0".to_string()),
        ]
    );
}

#[test]
fn gradle_lockfile_malformed_and_conflicting_fail_closed() {
    let err = parse_gradle_lockfile("only:two\n").expect_err("test operation must fail");
    assert!(err.to_string().contains("gradle.lockfile line 1"), "{err}");
    let conflicting = parse_gradle_lockfile("a.b:c:1.0.0:x\na.b:c:2.0.0:y\n")
        .expect("test operation must succeed");
    let err = collapse_gradle_pins(&conflicting).expect_err("test operation must fail");
    assert!(err.to_string().contains("refusing to guess"), "{err}");
}

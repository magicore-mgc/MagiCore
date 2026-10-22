//! Tests for Gradle version-catalog pin mutations.
//! Test thao tác cập nhật pin version catalog Gradle.

use super::super::bump_catalog_pin;
use crate::manifest::gradle::error::GradleManifestError;
use std::fs;

#[test]
fn bumps_inline_and_reference_pins_and_preserves_comments() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let catalog_dir = directory.path().join("gradle");
    fs::create_dir_all(&catalog_dir).expect("create catalog directory");
    let path = catalog_dir.join("libs.versions.toml");
    fs::write(
        &path,
        r#"# Keep this note.
[versions]
shared = "1.0.0"

[libraries]
ref = { module = "org.example:ref", version.ref = "shared" }
inline = "org.example:inline:1.0.0"
"#,
    )
    .expect("write catalog");

    assert!(bump_catalog_pin(directory.path(), "org.example", "ref", "2.0.0").unwrap());
    assert!(bump_catalog_pin(directory.path(), "org.example", "inline", "3.0.0").unwrap());
    let updated = fs::read_to_string(path).expect("read updated catalog");
    assert!(updated.contains("# Keep this note."));
    assert!(updated.contains("shared = \"2.0.0\""));
    assert!(updated.contains("org.example:inline:3.0.0"));
}

#[test]
fn shared_reference_refusal_leaves_catalog_unchanged() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let catalog_dir = directory.path().join("gradle");
    fs::create_dir_all(&catalog_dir).expect("create catalog directory");
    let path = catalog_dir.join("libs.versions.toml");
    let original = r#"[versions]
shared = "1.0.0"
[libraries]
first = { module = "org.example:first", version.ref = "shared" }
second = { module = "org.example:second", version.ref = "shared" }
"#;
    fs::write(&path, original).expect("write catalog");

    let error = bump_catalog_pin(directory.path(), "org.example", "first", "2.0.0")
        .expect_err("shared reference must be refused");
    assert!(matches!(
        error,
        GradleManifestError::Unsupported {
            capability: "bump-shared-gradle-version-ref",
            ..
        }
    ));
    assert_eq!(
        fs::read_to_string(path).expect("read original catalog"),
        original
    );
}

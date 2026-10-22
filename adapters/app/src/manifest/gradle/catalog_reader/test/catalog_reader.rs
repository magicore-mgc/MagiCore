//! Tests for supported Gradle version-catalog shapes.
//! Test các dạng version catalog Gradle được hỗ trợ.

use super::super::parse_catalog_content;

#[test]
fn parses_literal_and_reference_pins_and_skips_unresolved_entries() {
    let source = r#"
[versions]
shared = "2.1.0"
[libraries]
literal = "org.example:literal:1.2.3"
referenced = { module = "org.example:referenced", version.ref = "shared" }
unresolved = { module = "org.example:unresolved", version.ref = "missing" }
no-version = { module = "org.example:no-version" }
"#;
    let entries = parse_catalog_content(source).expect("catalog shape is supported");

    assert_eq!(entries.len(), 2);
    assert!(entries.iter().any(|entry| {
        entry.alias == "literal"
            && entry.group == "org.example"
            && entry.artifact == "literal"
            && entry.version == "1.2.3"
            && entry.version_ref.is_none()
    }));
    assert!(entries.iter().any(|entry| {
        entry.alias == "referenced"
            && entry.version == "2.1.0"
            && entry.version_ref.as_deref() == Some("shared")
    }));
}

#[test]
fn malformed_toml_or_missing_required_tables_returns_none() {
    assert!(parse_catalog_content("[libraries\n").is_none());
    assert!(
        parse_catalog_content("[libraries]\nexample = \"org.example:example:1.0\"\n").is_none()
    );
}

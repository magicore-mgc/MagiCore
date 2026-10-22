//! Tests for recursive version-reference counting.
//! Test đếm version reference đệ quy.

use super::super::count_version_ref_uses;

#[test]
fn counts_matching_references_in_nested_tables_and_arrays() {
    let value: toml::Value = toml::from_str(
        r#"[versions]
shared = "1"
[[libraries]]
name = "one"
[libraries.version]
ref = "shared"
[[libraries]]
name = "two"
[libraries.version]
ref = "shared"
"#,
    )
    .expect("parse nested TOML tables");

    assert_eq!(count_version_ref_uses(&value, "shared"), 2);
}

#[test]
fn ignores_other_refs_and_non_reference_values() {
    let value: toml::Value = toml::from_str(
        r#"[versions]
shared = "1"
other = "2"
[libraries]
inline = "org.example:inline:1"
ref = { module = "org.example:ref", version.ref = "other" }
"#,
    )
    .expect("parse TOML");

    assert_eq!(count_version_ref_uses(&value, "shared"), 0);
}

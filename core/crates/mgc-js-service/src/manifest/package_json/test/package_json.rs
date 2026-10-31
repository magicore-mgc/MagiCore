//! PackageJson schema round-trip tests.
//! Test round-trip schema PackageJson.

use super::super::PackageJson;

#[test]
fn omitted_project_version_stays_absent_but_explicit_empty_version_is_invalid() {
    let package: PackageJson = serde_json::from_str(r#"{"name":"private-app","private":true}"#)
        .expect("unversioned local application");
    assert!(package.version.is_empty());
    assert!(
        serde_json::to_value(package)
            .expect("serialize application")
            .get("version")
            .is_none()
    );
    for version in [
        serde_json::json!(""),
        serde_json::Value::Null,
        serde_json::json!(42),
    ] {
        assert!(
            serde_json::from_value::<PackageJson>(serde_json::json!({
                "name": "private-app", "version": version
            }))
            .is_err()
        );
    }
}

#[test]
fn package_json_round_trip_preserves_unknown_fields() {
    let input = r#"{"name":"demo","version":"1.0.0","custom":{"enabled":true}}"#;
    let package: PackageJson = serde_json::from_str(input).expect("deserialize package");
    let output = serde_json::to_value(package).expect("serialize package");

    assert_eq!(output["custom"]["enabled"], serde_json::Value::Bool(true));
    assert_eq!(output["name"], "demo");
    assert_eq!(output["version"], "1.0.0");
}

/// `mgc update` dropped the trailing newline of package.json: before the
/// fix the file ended `}\n}` instead of `}\n}\n`.
/// (`mgc update` làm mất newline cuối của package.json: trước khi sửa file
/// kết thúc `}\n}` thay vì `}\n}\n`.)
#[test]
fn serialized_manifest_ends_with_exactly_one_newline() {
    let package: PackageJson = serde_json::from_str(
        r#"{"name":"demo","version":"1.0.0","dependencies":{"lodash":"^4.17.21"}}"#,
    )
    .expect("deserialize package");
    let content = crate::manifest::package_json::serialize_package_json(&package)
        .expect("serialize package must succeed");
    assert!(
        content.ends_with("}\n"),
        "serialized package.json must end with a newline, got: {content:?}"
    );
    assert!(
        !content.ends_with("}\n\n"),
        "exactly one trailing newline is expected, got: {content:?}"
    );
    assert!(!content.trim_end_matches('\n').ends_with('\n'));
}

#[test]
fn saving_rewrites_a_manifest_without_a_trailing_newline() {
    let dir = tempfile::TempDir::new().expect("test operation must succeed");
    let path = dir.path().join("package.json");
    std::fs::write(
        &path,
        "{\n  \"name\": \"demo\",\n  \"version\": \"1.0.0\"\n}",
    )
    .expect("test operation must succeed");
    let mut package = PackageJson::load(&path).expect("load package");
    package
        .dependencies
        .get_or_insert_with(std::collections::HashMap::new)
        .insert("lodash".to_string(), "^4.17.21".to_string());
    package.save(&path).expect("save package");
    let written = std::fs::read_to_string(&path).expect("read back package");
    assert!(
        written.ends_with("}\n"),
        "rewritten manifest must keep a trailing newline, got: {written:?}"
    );
    // Saving an unchanged manifest again must not rewrite/drift the bytes.
    package.save(&path).expect("save package again");
    assert_eq!(
        std::fs::read_to_string(&path).expect("read back package"),
        written
    );
}

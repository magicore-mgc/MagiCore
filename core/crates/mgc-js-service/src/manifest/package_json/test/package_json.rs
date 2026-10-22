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

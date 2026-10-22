// Package declaration lookup tests.
// Test tìm khai báo package.

use serde_json::json;

use super::declared_version;

#[test]
fn dependency_sections_keep_the_existing_priority() {
    let manifest = json!({
        "dependencies": {"target": "^1.0"},
        "devDependencies": {"target": "^2.0"},
        "peerDependencies": {"target": "^3.0"},
        "optionalDependencies": {"target": "^4.0"}
    });
    assert_eq!(declared_version(&manifest, "target").as_deref(), Some("^1.0"));
}

#[test]
fn falls_through_to_later_sections_and_ignores_non_string_values() {
    let manifest = json!({
        "dependencies": {"target": null},
        "devDependencies": "not-an-object",
        "peerDependencies": {"target": "~3.2"}
    });
    assert_eq!(declared_version(&manifest, "target").as_deref(), Some("~3.2"));
}

#[test]
fn returns_none_when_package_is_not_declared() {
    let manifest = json!({"dependencies": {"other": "1.0.0"}});
    assert_eq!(declared_version(&manifest, "target"), None);
}

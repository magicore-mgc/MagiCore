//! Focused npm package-version wire-format tests.
//! Test riêng định dạng dữ liệu version package npm.

use crate::model::PackageVersion;

#[test]
fn test_version_keeps_camel_case_ids_and_optional_script_omission() {
    let version: PackageVersion = serde_json::from_value(serde_json::json!({
        "name": "pkg",
        "version": "1.0.0",
        "dist": {
            "integrity": "sha512-value",
            "shasum": "sha1-value",
            "tarball": "https://example.invalid/pkg.tgz"
        },
        "devDependencies": { "dep": "^1" },
        "_id": "pkg@1.0.0",
        "_rev": "7"
    }))
    .unwrap();

    assert_eq!(
        version
            .dev_dependencies
            .as_ref()
            .unwrap()
            .get("dep")
            .map(String::as_str),
        Some("^1")
    );
    let json = serde_json::to_value(version).unwrap();
    assert_eq!(json["devDependencies"]["dep"], "^1");
    assert_eq!(json["_id"], "pkg@1.0.0");
    assert!(json.get("scripts").is_none());
}

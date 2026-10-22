//! Focused npm package wire-format tests.
//! Test riêng định dạng dữ liệu package npm.

use crate::model::Package;

#[test]
fn test_package_keeps_dist_tags_and_defaulted_maintainers() {
    let package: Package = serde_json::from_value(serde_json::json!({
        "name": "@scope/pkg",
        "versions": {},
        "dist-tags": { "latest": "1.0.0" },
        "maintainers": [{ "name": "Example" }]
    }))
    .unwrap();

    assert_eq!(
        package.dist_tags.get("latest").map(String::as_str),
        Some("1.0.0")
    );
    assert_eq!(package.maintainers[0].email, None);
    let json = serde_json::to_value(package).unwrap();
    assert_eq!(json["dist-tags"]["latest"], "1.0.0");
}

//! Tests the serialized outdated result contract.
//! Kiểm tra contract serialize của kết quả outdated.

#![allow(clippy::unwrap_used)]

use super::super::OutdatedPkg;
use serde_json::json;

#[test]
fn outdated_package_keeps_the_public_json_fields() {
    let package = OutdatedPkg {
        name: "widget".to_string(),
        current: "^1.2.0".to_string(),
        latest: "2.0.0".to_string(),
        major: 2,
        minor: 0,
        patch: 0,
    };
    assert_eq!(
        serde_json::to_value(package).unwrap(),
        json!({
            "name": "widget",
            "current": "^1.2.0",
            "latest": "2.0.0",
            "major": 2,
            "minor": 0,
            "patch": 0
        })
    );
}

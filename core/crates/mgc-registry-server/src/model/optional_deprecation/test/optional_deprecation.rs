//! Focused tests for accepted npm deprecation input shapes.
//! Test riêng các dạng đầu vào deprecated mà npm chấp nhận.

use crate::model::PackageVersion;

fn parse_version(deprecated: serde_json::Value) -> PackageVersion {
    serde_json::from_value(serde_json::json!({
        "name": "pkg",
        "version": "1.0.0",
        "dist": {
            "integrity": "sha512-value",
            "shasum": "sha1-value",
            "tarball": "https://example.invalid/pkg.tgz"
        },
        "deprecated": deprecated
    }))
    .unwrap()
}

#[test]
fn test_boolean_deprecation_input_is_accepted() {
    assert!(parse_version(serde_json::json!(false)).deprecated.is_some());
}

#[test]
fn test_string_deprecation_input_is_accepted() {
    assert_eq!(
        parse_version(serde_json::json!("reason"))
            .deprecated
            .as_deref(),
        Some("")
    );
}

#[test]
fn test_null_deprecation_input_is_absent() {
    assert!(parse_version(serde_json::Value::Null).deprecated.is_none());
}

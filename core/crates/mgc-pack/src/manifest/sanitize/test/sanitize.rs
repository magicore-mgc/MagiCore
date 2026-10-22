//! Tests for manifest sanitization and validation boundaries.
//! Test xử lý sanitize và ranh giới validation manifest.

use super::sanitize;
use crate::manifest::ManifestSanitizeError;

#[test]
fn rejects_non_object_input_with_the_existing_diagnostic() {
    let error = sanitize(serde_json::json!(["not", "an object"]), "", "")
        .err()
        .expect("non-object manifests must be rejected");

    assert_eq!(error.to_string(), "manifest is not a JSON object");
}

#[test]
fn rejects_private_packages_before_projection() {
    let manifest = serde_json::json!({"name":"pkg","version":"1.0.0","private":true});
    let error = sanitize(manifest, "", "")
        .err()
        .expect("private packages must not be published");

    assert_eq!(
        error.to_string(),
        "package.json has private: true — cannot publish (remove private: true to publish)"
    );
}

#[test]
fn rejects_malformed_private_values_instead_of_stripping_them() {
    for private in [
        serde_json::json!("true"),
        serde_json::json!(1),
        serde_json::Value::Null,
    ] {
        let manifest = serde_json::json!({
            "name":"pkg", "version":"1.0.0", "private":private
        });
        let error = sanitize(manifest, "", "")
            .err()
            .expect("non-boolean private values must fail closed");

        assert_eq!(error.to_string(), "package.json private must be a boolean");
        assert!(error.downcast_ref::<ManifestSanitizeError>().is_some());
    }
}

#[test]
fn removes_publish_only_fields_and_keeps_other_manifest_data() {
    let manifest = serde_json::json!({
        "name":"pkg", "version":"1.0.0", "private":false,
        "publishConfig":{"registry":"https://example.invalid"},
        "main":"index.js", "exports":{".":"./index.js"}
    });
    let sanitized = sanitize(manifest, "", "").expect("valid public package should sanitize");

    assert!(sanitized.manifest.get("private").is_none());
    assert!(sanitized.manifest.get("publishConfig").is_none());
    assert_eq!(sanitized.manifest["main"], "index.js");
    assert_eq!(sanitized.manifest["exports"]["."], "./index.js");
}

//! Tests for the public sanitized result type.
//! Test cho type kết quả đã làm sạch.

use super::Sanitized;

#[test]
fn result_retains_manifest_and_publish_identity() {
    let result = Sanitized {
        manifest: serde_json::json!({"main": "index.js"}),
        name: "sample".to_owned(),
        version: "1.2.3".to_owned(),
    };

    assert_eq!(result.manifest["main"], "index.js");
    assert_eq!(result.name, "sample");
    assert_eq!(result.version, "1.2.3");
}

//! Verify the serialized fields of the publish summary.
//! Kiểm tra các field được serialize của publish summary.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::PublishSummary;

#[test]
fn serializes_all_current_summary_fields() {
    let summary = PublishSummary {
        name: "demo".into(),
        version: "1.2.3".into(),
        tag: "latest".into(),
        size: 101,
        unpacked_size: 303,
        shasum: "sha1-demo".into(),
        integrity: "sha512-demo".into(),
        files: 4,
        entry_count: 5,
    };
    let value = serde_json::to_value(summary).unwrap();
    let expected = serde_json::json!({
        "name": "demo",
        "version": "1.2.3",
        "tag": "latest",
        "size": 101,
        "unpacked_size": 303,
        "shasum": "sha1-demo",
        "integrity": "sha512-demo",
        "files": 4,
        "entry_count": 5
    });

    assert_eq!(value, expected);
}

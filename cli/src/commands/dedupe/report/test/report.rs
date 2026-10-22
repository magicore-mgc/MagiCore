//! Dedupe report tests — protects serialized fields used by JSON consumers.
//! Test report dedupe — bảo vệ field serialize cho consumer JSON.
use super::super::report_json;
use crate::commands::dedupe::{DedupeEntry, DedupeReport};

#[test]
fn json_report_keeps_existing_count_and_gc_fields() {
    let report = DedupeReport {
        before_instances: 3,
        after_instances: 2,
        merged: 1,
        disk_saved_bytes: 4,
        entries: vec![DedupeEntry {
            package: "probe".to_string(),
            version: "1.0.0".to_string(),
            instances_before: 2,
            instances_after: 1,
            action: "merged".to_string(),
        }],
        gc_failed_paths: vec!["/tmp/orphan: denied".to_string()],
    };

    let json = report_json(&report).unwrap();
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(value["before_instances"], 3);
    assert_eq!(value["after_instances"], 2);
    assert_eq!(value["merged"], 1);
    assert_eq!(value["disk_saved_bytes"], 4);
    assert_eq!(value["entries"][0]["package"], "probe");
    assert_eq!(value["gc_failed_paths"][0], "/tmp/orphan: denied");
}

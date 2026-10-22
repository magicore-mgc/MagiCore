#![allow(clippy::unwrap_used)]
//! Metadata serialization tests — Test tuần tự hóa metadata

use super::super::super::*;

#[test]
fn omits_an_absent_component_and_keeps_camel_case_fields() {
    let metadata = Metadata {
        timestamp: "2026-10-17T00:00:00Z".into(),
        tools: Vec::new(),
        component: None,
    };
    let value = serde_json::to_value(metadata).unwrap();

    assert_eq!(value["timestamp"], "2026-10-17T00:00:00Z");
    assert_eq!(value["tools"], serde_json::json!([]));
    assert!(value.get("component").is_none());
}

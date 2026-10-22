//! Covers optional provenance values in serialized schema.
//! Kiểm tra giá trị provenance tùy chọn trong schema đã serialize.

use super::super::Provenance;

#[test]
fn omits_absent_tool_and_import_source() {
    let provenance = Provenance {
        source_kind: "native-resolve".to_string(),
        tool: None,
        imported_from: None,
    };

    let value = serde_json::to_value(&provenance).expect("test operation must succeed");
    assert_eq!(value["source_kind"], "native-resolve");
    assert!(value.get("tool").is_none());
    assert!(value.get("imported_from").is_none());
}

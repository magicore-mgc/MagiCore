//! Pin registry response fallback behavior.
//! Cố định hành vi fallback của response registry.

use serde_json::json;

use super::super::decode_search_response;

#[test]
fn extracts_total_and_objects_from_the_response_envelope() {
    let response = decode_search_response(json!({
        "total": 2,
        "objects": [{"package": {"name": "one"}}, {"package": {"name": "two"}}]
    }));

    assert_eq!(response.total, 2);
    assert_eq!(response.objects.len(), 2);
}

#[test]
fn preserves_zero_and_empty_fallbacks_for_malformed_fields() {
    let response = decode_search_response(json!({"total": "unknown", "objects": null}));

    assert_eq!(response.total, 0);
    assert!(response.objects.is_empty());
}

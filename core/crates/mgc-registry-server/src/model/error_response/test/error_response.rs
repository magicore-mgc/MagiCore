//! Focused registry error-response serialization tests.
//! Test riêng serialization response lỗi registry.

use crate::model::ErrorResponse;

#[test]
fn test_error_response_builder_omits_missing_details() {
    let response = ErrorResponse::new("bad_request", "invalid input");
    let json = serde_json::to_value(&response).unwrap();

    assert_eq!(response.error, "bad_request");
    assert_eq!(response.message, "invalid input");
    assert!(json.get("details").is_none());
}

#[test]
fn test_error_response_builder_serializes_details() {
    let response = ErrorResponse::new("invalid", "bad field")
        .with_details(serde_json::json!({ "field": "name" }));
    let json = serde_json::to_value(response).unwrap();

    assert_eq!(json["details"]["field"], "name");
}

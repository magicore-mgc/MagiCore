//! Pin transparent display and underlying error sources.
//! Cố định display transparent và source lỗi gốc.

use std::error::Error;

use super::super::SearchError;

#[test]
fn preserves_registry_error_display_and_source() {
    let source = anyhow::anyhow!("registry configuration rejected");
    let expected = source.to_string();
    let error = SearchError::Registry(source);

    assert_eq!(error.to_string(), expected);
    assert!(error.source().is_some());
}

#[test]
fn preserves_serialization_error_display_and_source() {
    let source = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
    let expected = source.to_string();
    let error = SearchError::Serialization(source);

    assert_eq!(error.to_string(), expected);
    assert!(error.source().is_some());
}

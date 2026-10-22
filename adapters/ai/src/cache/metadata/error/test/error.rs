//! Metadata error cases — kiểm tra source chain và diagnostic typed.

#![cfg(test)]

use super::error::MetadataError;
use mgc_types::MgError;

#[test]
fn io_error_preserves_operation_and_source() {
    let error = MgError::from(MetadataError::io(
        "sync metadata sidecar",
        std::io::Error::other("fixture I/O failure"),
    ));

    assert!(error.to_string().contains("sync metadata sidecar"));
    assert!(std::error::Error::source(&error).is_some());
}

#[test]
fn malformed_json_error_preserves_serde_source() {
    let source = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
    let error = MgError::from(MetadataError::Deserialize(source));

    assert!(error.to_string().contains("parse AI cache metadata"));
    assert!(std::error::Error::source(&error).is_some());
}

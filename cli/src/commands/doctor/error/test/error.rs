//! Verifies that JSON failures retain their source error.
//! Xác nhận lỗi JSON giữ lại source error.

use std::error::Error as _;

use super::super::DoctorError;

#[test]
fn json_error_preserves_serializer_source() {
    let source = serde_json::from_str::<u8>("invalid").unwrap_err();
    let error = DoctorError::from(source);
    assert!(error.source().is_some());
}

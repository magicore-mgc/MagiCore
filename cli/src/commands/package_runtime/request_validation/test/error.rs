//! Request validation error contract regression.
//! Test hồi quy contract lỗi request validation.

use super::*;

#[test]
fn invalid_request_is_typed_and_keeps_its_safe_reason() {
    let error = registry_package_invalid_request("invalid registry package name");
    assert_eq!(
        error.to_string(),
        "package runner: invalid registry package request: invalid registry package name"
    );
    assert!(matches!(
        error.downcast_ref::<RequestValidationError>(),
        Some(RequestValidationError::InvalidRequest { reason }) if reason == "invalid registry package name"
    ));
}

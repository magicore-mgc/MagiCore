//! Upload error mapping and source tests.
//! Kiểm thử ánh xạ và source lỗi upload.

use super::super::error::UploadLegacyError;
use axum::{http::StatusCode, response::IntoResponse};
use std::error::Error;

#[test]
fn storage_and_signing_causes_are_retained_with_their_statuses() {
    let storage = UploadLegacyError::Storage(anyhow::anyhow!("database failure"));
    assert!(Error::source(&storage).is_some());
    assert_eq!(storage.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        storage.into_response().status(),
        StatusCode::INTERNAL_SERVER_ERROR
    );

    let signing = UploadLegacyError::Signing(anyhow::anyhow!("signing failure"));
    assert!(Error::source(&signing).is_some());
    assert_eq!(signing.status_code(), StatusCode::SERVICE_UNAVAILABLE);
}

#[test]
fn digest_mismatch_keeps_bad_request_status_without_echoing_digest() {
    let error = UploadLegacyError::DigestMismatch;
    assert_eq!(error.status_code(), StatusCode::BAD_REQUEST);
    assert_eq!(
        error.to_string(),
        "PyPI upload SHA-256 digest did not match"
    );
}

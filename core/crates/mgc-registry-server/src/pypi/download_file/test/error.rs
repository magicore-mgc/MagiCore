//! Download error mapping and source tests.
//! Kiểm thử ánh xạ và source lỗi download.

use super::super::error::DownloadFileError;
use axum::{
    http::{HeaderValue, StatusCode},
    response::IntoResponse,
};
use std::error::Error;

#[test]
fn storage_failure_keeps_its_source_and_internal_status() {
    let error = DownloadFileError::StorageRead(anyhow::anyhow!("database failure"));
    assert!(Error::source(&error).is_some());
    assert_eq!(error.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        error.into_response().status(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[test]
fn invalid_header_and_missing_file_keep_existing_statuses() {
    let invalid = HeaderValue::from_str("\r\n").unwrap_err();
    let error = DownloadFileError::InvalidHeaderValue(invalid);
    assert!(Error::source(&error).is_some());
    assert_eq!(error.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        DownloadFileError::FileNotFound.status_code(),
        StatusCode::NOT_FOUND
    );
}

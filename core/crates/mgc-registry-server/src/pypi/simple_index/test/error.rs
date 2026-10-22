//! Simple-index typed error tests.
//! Kiểm thử lỗi có kiểu của simple-index.

use super::super::error::SimpleIndexError;
use axum::{http::StatusCode, response::IntoResponse};
use std::error::Error;

#[test]
fn storage_source_is_retained_and_maps_to_internal_error() {
    let error = SimpleIndexError::StorageRead(anyhow::anyhow!("database failure"));
    assert!(Error::source(&error).is_some());
    assert_eq!(error.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        error.into_response().status(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[test]
fn request_and_missing_package_statuses_remain_unchanged() {
    assert_eq!(
        SimpleIndexError::from(StatusCode::UNAUTHORIZED).status_code(),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        SimpleIndexError::PackageNotFound.status_code(),
        StatusCode::NOT_FOUND
    );
}

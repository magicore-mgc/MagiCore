//! Tests for local error classification and source preservation.
//! Test phân loại lỗi cục bộ và giữ nguyên source.

use std::io;
use std::time::{SystemTimeError, UNIX_EPOCH};

use super::SearchCacheError;

#[test]
fn io_error_keeps_its_public_type() {
    let source = io::Error::other("cache io");
    let error = SearchCacheError::Io(source).into_anyhow();

    assert!(error.downcast_ref::<io::Error>().is_some());
    assert_eq!(error.to_string(), "cache io");
}

#[test]
fn clock_error_keeps_its_public_type() {
    let source = UNIX_EPOCH
        .duration_since(UNIX_EPOCH + std::time::Duration::from_secs(1))
        .expect_err("a time before UNIX_EPOCH must be rejected");
    let error = SearchCacheError::Clock(source).into_anyhow();

    assert!(error.downcast_ref::<SystemTimeError>().is_some());
}

#[test]
fn json_error_keeps_its_public_type() {
    let source = serde_json::from_str::<serde_json::Value>("!")
        .expect_err("invalid JSON must return a parse error");
    let error = SearchCacheError::Json(source).into_anyhow();

    assert!(error.downcast_ref::<serde_json::Error>().is_some());
}

#[test]
fn sqlite_error_keeps_its_public_type() {
    let error = SearchCacheError::Sqlite(rusqlite::Error::InvalidQuery).into_anyhow();

    assert!(error.downcast_ref::<rusqlite::Error>().is_some());
}

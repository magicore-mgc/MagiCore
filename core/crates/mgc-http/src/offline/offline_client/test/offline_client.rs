// OfflineClient test cases — nearest-owner cache and typed-error coverage.
// Các ca kiểm thử OfflineClient — kiểm tra cache và lỗi typed tại owner gần nhất.
#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! OfflineClient owner tests — kiểm thử owner OfflineClient.

use super::{OfflineClient, OfflineError};
use std::time::{Duration, SystemTime};

#[test]
fn offline_client_cache_hit_fresh() {
    let mut client = OfflineClient::new(Duration::from_secs(600));
    client
        .cache
        .insert("test".into(), (b"data".to_vec(), SystemTime::now()));
    let rt = tokio::runtime::Runtime::new().unwrap();
    let data = rt.block_on(client.get("test")).unwrap();
    assert_eq!(data, b"data");
}

#[test]
fn offline_client_stale_warning() {
    let mut client = OfflineClient::new(Duration::from_secs(1));
    let past = SystemTime::now() - Duration::from_secs(10);
    client.cache.insert("test".into(), (b"data".to_vec(), past));
    let rt = tokio::runtime::Runtime::new().unwrap();
    let data = rt.block_on(client.get("test")).unwrap();
    assert_eq!(data, b"data"); // still returns data but prints warning — vẫn trả dữ liệu và in cảnh báo
}

#[test]
fn offline_client_miss_fails() {
    let client = OfflineClient::new(Duration::from_secs(600));
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(client.get("missing"));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("E_NET_OFFLINE"));
}

#[test]
fn offline_error_preserves_display_and_source() {
    let source = std::io::Error::new(std::io::ErrorKind::InvalidInput, "bad cache");
    let error = OfflineError::ReadCache(source);
    assert_eq!(error.to_string(), "read cache file: bad cache");
    assert!(std::error::Error::source(&error).is_some());
}

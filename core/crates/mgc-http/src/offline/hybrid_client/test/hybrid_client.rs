// HybridClient test cases — deterministic mode selection without network.
// Các ca kiểm thử HybridClient — chọn mode xác định, không dùng mạng.
#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! HybridClient owner tests — kiểm thử owner HybridClient.

use super::HybridClient;
use std::time::{Duration, SystemTime};

#[test]
fn explicit_offline_mode_reads_seeded_cache_without_network() {
    let mut client = HybridClient::new(Duration::from_secs(600)).unwrap();
    client
        .offline
        .cache
        .insert("test".into(), (b"offline".to_vec(), SystemTime::now()));
    client.set_offline(true);

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let response = runtime.block_on(client.get("test")).unwrap();

    assert_eq!(response, b"offline");
    assert!(client.offline_mode);
}

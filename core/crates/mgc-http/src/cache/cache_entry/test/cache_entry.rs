// CacheEntry tests — retain the existing TTL boundaries.
// Test CacheEntry — giữ nguyên các mốc TTL hiện có.
#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! CacheEntry owner tests — kiểm thử owner CacheEntry.

use super::*;
use std::time::{Duration, SystemTime};

#[test]
fn cache_entry_fresh_is_valid() {
    let entry = CacheEntry {
        data: vec![1, 2, 3],
        timestamp: SystemTime::now(),
        ttl: Duration::from_secs(60),
    };
    assert!(entry.is_valid());
}

#[test]
fn cache_entry_expired_is_invalid() {
    let past = SystemTime::now()
        .checked_sub(Duration::from_secs(120))
        .expect("system time anomaly");
    let entry = CacheEntry {
        data: Vec::new(),
        timestamp: past,
        ttl: Duration::from_secs(60),
    };
    assert!(!entry.is_valid());
}

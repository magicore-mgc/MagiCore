//! Age cases — kiểm tra số ngày trọn và timestamp tương lai.

#![cfg(test)]

use crate::cache::metadata::CacheEntry;
use std::time::{Duration, SystemTime};

#[test]
fn current_and_future_timestamps_have_zero_elapsed_days() {
    let now = SystemTime::now();
    let entry = CacheEntry {
        model_id: "test".into(),
        size_bytes: 0,
        cached_at: now,
        last_accessed: now + Duration::from_secs(86_400),
        checksum: None,
    };

    assert_eq!(entry.age_days(), 0);
    assert_eq!(entry.days_since_access(), 0);
}

#[test]
fn age_discards_partial_days() {
    let entry = CacheEntry {
        model_id: "test".into(),
        size_bytes: 0,
        cached_at: SystemTime::now() - Duration::from_secs(86_400 + 86_399),
        last_accessed: SystemTime::now() - Duration::from_secs(86_400 + 86_399),
        checksum: None,
    };

    assert_eq!(entry.age_days(), 1);
    assert_eq!(entry.days_since_access(), 1);
}

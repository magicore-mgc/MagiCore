//! Admit one registry cache entry without exceeding its configured entry count.
//! Ghi một entry registry cache nhưng không vượt số lượng đã cấu hình.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use dashmap::DashMap;

use super::super::cache_entry::CacheEntry;

pub(in crate::cache) fn insert_bounded<T>(
    entries: &DashMap<String, Arc<CacheEntry<T>>>,
    insert_gate: &Mutex<()>,
    key: String,
    value: Vec<T>,
    ttl: Duration,
    max_entries: usize,
) {
    let _guard = insert_gate
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    entries.retain(|_, entry| entry.is_fresh(ttl));
    if max_entries == 0 {
        return;
    }

    let entry = Arc::new(CacheEntry::new(value));
    if entries.contains_key(&key) {
        entries.insert(key, entry);
        return;
    }

    if entries.len() >= max_entries {
        let oldest_key = entries
            .iter()
            .min_by_key(|entry| entry.fetched_at())
            .map(|entry| entry.key().clone());
        if let Some(oldest_key) = oldest_key {
            entries.remove(&oldest_key);
        }
    }
    entries.insert(key, entry);
}

// HttpCache tests — insertion, shared clone and expiration.
// Test HttpCache — insert, chia sẻ clone và hết hạn.
#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::HttpCache;
use std::time::Duration;

#[test]
fn inserted_payload_is_returned_while_fresh() {
    let cache = HttpCache::new();
    cache.insert(
        "registry/item".to_string(),
        vec![1, 2, 3],
        Duration::from_secs(60),
    );

    assert_eq!(cache.get("registry/item"), Some(vec![1, 2, 3]));
}

#[test]
fn cache_clone_shares_inserted_entries() {
    let cache = HttpCache::new();
    let clone = cache.clone();
    cache.insert(
        "registry/item".to_string(),
        vec![4, 5],
        Duration::from_secs(60),
    );

    assert_eq!(clone.get("registry/item"), Some(vec![4, 5]));
}

#[test]
fn zero_ttl_entry_is_a_miss() {
    let cache = HttpCache::new();
    cache.insert("registry/item".to_string(), vec![6], Duration::ZERO);

    assert_eq!(cache.get("registry/item"), None);
}

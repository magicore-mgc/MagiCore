// HttpCache owner — shared in-memory cache with poison recovery.
// Owner HttpCache — cache dùng chung trong bộ nhớ, phục hồi lock bị poison.

use super::super::CacheEntry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

/// Thread-shareable in-memory HTTP response cache.
/// Cache response HTTP trong bộ nhớ, có thể chia sẻ giữa thread.
#[derive(Debug, Clone)]
pub struct HttpCache {
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
}

impl HttpCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        // Recover a poisoned lock; a miss or stale entry is safer than panicking readers.
        // Phục hồi lock poison; cache miss hoặc entry cũ an toàn hơn panic reader.
        let cache = self
            .cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cache.get(key).and_then(|entry| {
            if entry.is_valid() {
                Some(entry.data.clone())
            } else {
                None
            }
        })
    }

    pub fn insert(&self, key: String, data: Vec<u8>, ttl: Duration) {
        // Use the same recovery policy as get.
        // Dùng cùng chính sách phục hồi lock như get.
        let mut cache = self
            .cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cache.insert(
            key,
            CacheEntry {
                data,
                timestamp: SystemTime::now(),
                ttl,
            },
        );
    }
}

impl Default for HttpCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "test/http_cache.rs"]
mod tests;

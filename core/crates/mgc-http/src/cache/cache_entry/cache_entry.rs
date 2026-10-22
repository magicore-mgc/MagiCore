// CacheEntry owner — TTL validity for one in-memory response.
// Owner CacheEntry — kiểm tra TTL cho một response trong bộ nhớ.

use std::time::{Duration, SystemTime};

/// Cached HTTP payload with its original timestamp and TTL.
/// Payload HTTP trong cache cùng timestamp và TTL ban đầu.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub data: Vec<u8>,
    pub timestamp: SystemTime,
    pub ttl: Duration,
}

impl CacheEntry {
    pub fn is_valid(&self) -> bool {
        SystemTime::now()
            .duration_since(self.timestamp)
            .map(|elapsed| elapsed < self.ttl)
            .unwrap_or(false)
    }
}

#[cfg(test)]
#[path = "test/cache_entry.rs"]
mod tests;

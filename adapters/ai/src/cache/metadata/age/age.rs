//! Compute cache-entry age — tính tuổi entry cache theo timestamp.

use crate::cache::metadata::CacheEntry;
use std::time::SystemTime;

impl CacheEntry {
    /// Return whole days since the entry was cached.
    /// Trả số ngày trọn vẹn từ lúc entry được cache.
    pub fn age_days(&self) -> u64 {
        days_since(self.cached_at)
    }

    /// Return whole days since the entry was last accessed.
    /// Trả số ngày trọn vẹn từ lần cuối entry được truy cập.
    pub fn days_since_access(&self) -> u64 {
        days_since(self.last_accessed)
    }
}

fn days_since(timestamp: SystemTime) -> u64 {
    SystemTime::now()
        .duration_since(timestamp)
        .map(|duration| duration.as_secs() / 86_400)
        .unwrap_or(0)
}

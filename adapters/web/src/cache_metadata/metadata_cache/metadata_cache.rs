//! Own the process-local bounded metadata LRU.
//! Quản lý LRU metadata có giới hạn trong process.

use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use lru::LruCache;

use crate::native;

const MAX_METADATA_CACHE_ENTRIES: usize = 2048;
const METADATA_CACHE_TTL_SECS: u64 = 6 * 60 * 60;

pub struct MetadataCache {
    cache: Mutex<LruCache<String, (Arc<native::npm_registry::PackageMetadata>, Instant)>>,
    ttl: Duration,
}

impl MetadataCache {
    pub fn new() -> Self {
        let max_entries = std::env::var("MAGICORE_WEB_METADATA_CACHE_MAX_ENTRIES")
            .ok()
            .and_then(|v| v.trim().parse().ok())
            .unwrap_or(MAX_METADATA_CACHE_ENTRIES);
        let ttl_secs = std::env::var("MAGICORE_WEB_METADATA_CACHE_TTL_SECS")
            .ok()
            .and_then(|v| v.trim().parse().ok())
            .unwrap_or(METADATA_CACHE_TTL_SECS);
        Self {
            cache: Mutex::new(LruCache::new(
                NonZeroUsize::new(max_entries).unwrap_or(NonZeroUsize::MIN),
            )),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    fn guard(
        &self,
    ) -> MutexGuard<'_, LruCache<String, (Arc<native::npm_registry::PackageMetadata>, Instant)>>
    {
        self.cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn get(&self, key: &str) -> Option<Arc<native::npm_registry::PackageMetadata>> {
        let mut cache = self.guard();
        if let Some((meta, instant)) = cache.get(key) {
            if instant.elapsed() < self.ttl {
                return Some(Arc::clone(meta));
            } else {
                cache.pop(key);
            }
        }
        None
    }

    pub fn insert(&self, key: String, meta: Arc<native::npm_registry::PackageMetadata>) {
        let mut cache = self.guard();
        cache.put(key, (meta, Instant::now()));
    }
}

impl Default for MetadataCache {
    fn default() -> Self {
        Self::new()
    }
}

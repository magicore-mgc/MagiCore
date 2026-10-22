//! Immutable value and freshness metadata for one registry-cache entry.
//! Giá trị bất biến và metadata freshness của một entry registry cache.

use std::sync::Arc;
use std::time::{Duration, Instant};

pub(in crate::cache) struct CacheEntry<T> {
    value: Arc<[T]>,
    fetched_at: Instant,
}

impl<T> CacheEntry<T> {
    pub(in crate::cache) fn new(values: Vec<T>) -> Self {
        Self {
            value: values.into(),
            fetched_at: Instant::now(),
        }
    }

    pub(in crate::cache) fn is_fresh(&self, ttl: Duration) -> bool {
        self.fetched_at.elapsed() < ttl
    }

    pub(in crate::cache) fn fetched_at(&self) -> Instant {
        self.fetched_at
    }
}

impl<T: Clone> CacheEntry<T> {
    pub(in crate::cache) fn snapshot(&self) -> Vec<T> {
        self.value.to_vec()
    }
}

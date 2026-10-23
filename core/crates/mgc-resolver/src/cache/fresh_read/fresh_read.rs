//! Return a live entry snapshot and conditionally remove the observed expired entry.
//! Trả snapshot entry còn hạn và chỉ xóa entry hết hạn đã quan sát.

use std::sync::Arc;
use std::time::Duration;

use dashmap::DashMap;

use super::cache_entry::CacheEntry;

pub(in crate::cache) fn get_fresh<T: Clone>(
    entries: &DashMap<String, Arc<CacheEntry<T>>>,
    key: &str,
    ttl: Duration,
) -> Option<Vec<T>> {
    let entry = entries.get(key)?;
    if entry.is_fresh(ttl) {
        return Some(entry.snapshot());
    }

    let expired_entry = Arc::clone(entry.value());
    drop(entry);
    entries.remove_if(key, |_, current| Arc::ptr_eq(current, &expired_entry));
    None
}

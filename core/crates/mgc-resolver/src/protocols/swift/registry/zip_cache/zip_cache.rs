//! FIFO archive storage with independent byte and entry ceilings.
//! Lưu archive theo FIFO với giới hạn byte và số entry độc lập.

use super::cache_limits::SwiftZipCacheLimits;
use super::error::SwiftZipCacheError;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub(in crate::protocols::swift::registry) struct SwiftZipCache {
    limits: SwiftZipCacheLimits,
    state: Mutex<SwiftZipCacheState>,
}

#[derive(Debug, Default)]
struct SwiftZipCacheState {
    entries: HashMap<String, SwiftZipCacheEntry>,
    retained_bytes: usize,
    next_order: u64,
}

#[derive(Debug)]
struct SwiftZipCacheEntry {
    bytes: Vec<u8>,
    inserted_order: u64,
}

impl SwiftZipCache {
    pub(in crate::protocols::swift::registry) fn from_env() -> Self {
        Self::with_limits(SwiftZipCacheLimits::from_env())
    }

    fn with_limits(limits: SwiftZipCacheLimits) -> Self {
        Self {
            limits,
            state: Mutex::new(SwiftZipCacheState::default()),
        }
    }

    pub(in crate::protocols::swift::registry) fn insert(
        &self,
        key: String,
        bytes: Vec<u8>,
    ) -> mgc_types::MgResult<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| SwiftZipCacheError::LockPoisoned)?;
        remove_entry(&mut state, &key);
        let allocation_bytes = bytes.capacity();

        if self.limits.max_entries == 0
            || self.limits.max_bytes == 0
            || allocation_bytes > self.limits.max_bytes
        {
            return Ok(());
        }

        while state.entries.len() >= self.limits.max_entries
            || state.retained_bytes.saturating_add(allocation_bytes) > self.limits.max_bytes
        {
            let oldest = state
                .entries
                .iter()
                .min_by_key(|(_, entry)| entry.inserted_order)
                .map(|(entry_key, _)| entry_key.clone());
            let Some(oldest) = oldest else {
                break;
            };
            remove_entry(&mut state, &oldest);
        }

        if state.next_order == u64::MAX {
            state.entries.clear();
            state.retained_bytes = 0;
            state.next_order = 0;
        }
        let inserted_order = state.next_order;
        state.next_order += 1;
        state.retained_bytes += allocation_bytes;
        state.entries.insert(
            key,
            SwiftZipCacheEntry {
                bytes,
                inserted_order,
            },
        );
        Ok(())
    }

    pub(in crate::protocols::swift::registry) fn get(
        &self,
        key: &str,
    ) -> mgc_types::MgResult<Option<Vec<u8>>> {
        let state = self
            .state
            .lock()
            .map_err(|_| SwiftZipCacheError::LockPoisoned)?;
        Ok(state.entries.get(key).map(|entry| entry.bytes.clone()))
    }
}

fn remove_entry(state: &mut SwiftZipCacheState, key: &str) {
    if let Some(entry) = state.entries.remove(key) {
        if let Some(retained_bytes) = state.retained_bytes.checked_sub(entry.bytes.capacity()) {
            state.retained_bytes = retained_bytes;
        } else {
            state.entries.clear();
            state.retained_bytes = 0;
            state.next_order = 0;
        }
    }
}

impl Default for SwiftZipCache {
    fn default() -> Self {
        Self::with_limits(SwiftZipCacheLimits::default())
    }
}

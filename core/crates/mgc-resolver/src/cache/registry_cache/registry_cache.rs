//! Public cache lifecycle and version/dependency entry operations.
//! Vòng đời API cache và thao tác entry version/dependency.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use dashmap::DashMap;

use crate::cache::bounded_insert::insert_bounded;
use crate::cache::cache_entry::CacheEntry;
use crate::cache::fresh_read::get_fresh;
use crate::solver::ResolvedDep;

const DEFAULT_TTL: Duration = Duration::from_secs(300);
const DEFAULT_MAX_ENTRIES: usize = 2_048;

pub struct RegistryCache {
    versions: DashMap<String, Arc<CacheEntry<mgc_types::Version>>>,
    deps: DashMap<String, Arc<CacheEntry<ResolvedDep>>>,
    ttl: Duration,
    max_entries: usize,
    versions_insert_gate: Mutex<()>,
    deps_insert_gate: Mutex<()>,
}

impl Clone for RegistryCache {
    fn clone(&self) -> Self {
        Self {
            versions: self.versions.clone(),
            deps: self.deps.clone(),
            ttl: self.ttl,
            max_entries: self.max_entries,
            versions_insert_gate: Mutex::new(()),
            deps_insert_gate: Mutex::new(()),
        }
    }
}

impl RegistryCache {
    pub fn new() -> Self {
        Self::with_ttl_and_capacity(DEFAULT_TTL, DEFAULT_MAX_ENTRIES)
    }

    pub fn with_ttl(ttl: Duration) -> Self {
        Self::with_ttl_and_capacity(ttl, DEFAULT_MAX_ENTRIES)
    }

    /// Creates an independently bounded cache with an explicit TTL and per-map entry cap.
    /// Tạo cache độc lập có TTL và giới hạn entry riêng từng map.
    /// A zero cap disables retention; the cap counts entries, not their payload bytes.
    /// Capacity 0 tắt lưu; giới hạn đếm entry, không đếm byte payload.
    pub fn with_ttl_and_capacity(ttl: Duration, max_entries: usize) -> Self {
        Self {
            versions: DashMap::new(),
            deps: DashMap::new(),
            ttl,
            max_entries,
            versions_insert_gate: Mutex::new(()),
            deps_insert_gate: Mutex::new(()),
        }
    }

    pub fn get_versions(&self, name: &str) -> Option<Vec<mgc_types::Version>> {
        get_fresh(&self.versions, name, self.ttl)
    }

    pub fn insert_versions(&self, name: String, versions: Vec<mgc_types::Version>) {
        insert_bounded(
            &self.versions,
            &self.versions_insert_gate,
            name,
            versions,
            self.ttl,
            self.max_entries,
        );
    }

    pub fn get_deps(&self, version_key: &str) -> Option<Vec<ResolvedDep>> {
        get_fresh(&self.deps, version_key, self.ttl)
    }

    pub fn insert_deps(&self, version_key: String, deps: Vec<ResolvedDep>) {
        insert_bounded(
            &self.deps,
            &self.deps_insert_gate,
            version_key,
            deps,
            self.ttl,
            self.max_entries,
        );
    }

    pub fn clear(&self) {
        self.versions.clear();
        self.deps.clear();
    }

    pub fn len(&self) -> usize {
        self.versions.len() + self.deps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.versions.is_empty() && self.deps.is_empty()
    }

    pub fn evict_stale(&self) {
        self.versions.retain(|_, entry| entry.is_fresh(self.ttl));
        self.deps.retain(|_, entry| entry.is_fresh(self.ttl));
    }
}

impl Default for RegistryCache {
    fn default() -> Self {
        Self::new()
    }
}

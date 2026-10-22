//! `provider.rs` — npm dependency provider state and construction.
//! State và khởi tạo provider.

use super::super::release_age::AgePolicy;
use crate::cache::{MetadataCache, SharedWebCache};
use crate::native;
use dashmap::DashMap;
use mgc_resolver::RegistryCache;
use mgc_types::PackageName;
use std::sync::Arc;

pub struct NpmDependencyProvider {
    pub registry: native::npm_registry::NpmRegistry,
    pub metadata_cache: MetadataCache,
    pub metadata_locks: DashMap<String, Arc<tokio::sync::Mutex<()>>>,
    pub registry_cache: RegistryCache,
    pub shared_cache: Option<SharedWebCache>,
    pub alias_targets: DashMap<String, PackageName>,
    pub optional_enqueue_cache: DashMap<String, bool>,
    /// Per-instance age gate (P0/F6): armed per operation from that
    /// operation's project — never a process-global first-wins.
    /// (Cổng tuổi riêng từng instance.)
    age_policy: std::sync::RwLock<Option<AgePolicy>>,
}

impl NpmDependencyProvider {
    pub fn new(url: &str, token: Option<String>, shared_cache: Option<SharedWebCache>) -> Self {
        Self::new_with_chain(url, token, Vec::new(), shared_cache)
    }

    pub fn new_with_chain(
        url: &str,
        token: Option<String>,
        fallbacks: Vec<(String, Option<String>)>,
        shared_cache: Option<SharedWebCache>,
    ) -> Self {
        Self::from_registry(
            native::npm_registry::NpmRegistry::new_with_chain(url, token, fallbacks),
            shared_cache,
        )
    }

    /// Keep credential scopes in the provider-owned transport.
    /// Provider nhận chain typed, không bóc token để dựng lại quyền theo origin.
    pub fn new_with_scoped_chain(
        entries: Vec<mgc_config::chain::ScopedRegistry>,
        shared_cache: Option<SharedWebCache>,
    ) -> anyhow::Result<Self> {
        Ok(Self::from_registry(
            native::npm_registry::NpmRegistry::new_with_scoped_chain(entries)?,
            shared_cache,
        ))
    }

    fn from_registry(
        registry: native::npm_registry::NpmRegistry,
        shared_cache: Option<SharedWebCache>,
    ) -> Self {
        Self {
            registry,
            metadata_cache: MetadataCache::new(),
            metadata_locks: DashMap::new(),
            registry_cache: RegistryCache::new(),
            shared_cache,
            alias_targets: DashMap::new(),
            optional_enqueue_cache: DashMap::new(),
            age_policy: std::sync::RwLock::new(None),
        }
    }

    /// Overwrite this provider's gate (P0/F6 — every operation re-arms
    /// from its own project; no first-writer-wins). A policy change evicts
    /// version and optional-dependency decisions made under the old gate.
    /// Also flips the registry client's fetch mode (full packuments + no stale
    /// abbreviated cache) so fetchers and filters can never disagree.
    /// (Đặt lại cổng tuổi; đổi policy thì xóa quyết định version cũ.)
    pub fn set_age_policy(&self, policy: Option<AgePolicy>) {
        let armed = policy.is_some_and(|p| p.cutoff_hours > 0);
        let changed = {
            let mut slot = self
                .age_policy
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let changed = *slot != policy;
            *slot = policy;
            changed
        };
        if changed {
            self.registry_cache.clear();
            self.optional_enqueue_cache.clear();
        }
        self.registry.set_age_gate_armed(armed);
    }

    /// This provider's current policy (None = no filtering).
    /// (Policy hiện tại của provider này.)
    pub fn age_policy(&self) -> Option<AgePolicy> {
        self.age_policy.read().ok().and_then(|p| *p)
    }

    /// Test/scope probe: is any cutoff armed on this instance.
    /// (Probe: instance này có cutoff không.)
    pub fn age_gate_armed_for_test(&self) -> bool {
        self.age_policy().is_some_and(|p| p.cutoff_hours > 0)
    }
}

use super::DedupePref;
use crate::solver::provider::{DependencyProvider, ResolvedDep};
use mgc_types::Version;
use std::collections::HashMap;
use std::sync::Arc;

/// Batch resolver with error propagation.
///
/// ## Algorithm
/// 1. Enqueue initial deps from CLI.
/// 2. Drain batch (max 50), prefetch unique names.
/// 3. Per package:
///    a. Override check.
///    b. Already resolved → recheck, upgrade within major if needed.
///    c. Fresh resolve → pick latest matching version.
/// 4. Enqueue resolved deps.
/// 5. Repeat until queue empty.
///
/// ## Errors
/// Provider errors → immediate `SolveError`. No silent skips.
pub struct Resolver {
    pub(super) provider: std::sync::Arc<dyn DependencyProvider>,
    pub(super) overrides: HashMap<String, String>,
    /// Version selection preference (default PreferLatest = safe).
    /// RwLock so shared resolvers (Arc in adapters) can be reconfigured per solve.
    pub(super) dedupe_pref: std::sync::RwLock<DedupePref>,
    /// Versions already installed (from lockfile) to prefer under PreferExisting.
    pub(super) existing_versions: std::sync::RwLock<HashMap<String, Version>>,
    /// G2: Dependency memoization cache — caches resolved dependencies by PackageId.
    /// Provides ~2% speedup on warm installs by avoiding redundant dependency fetches.
    /// Key: "name@version" string, Value: Arc<[ResolvedDep]>
    pub(super) dep_memo_cache: std::sync::RwLock<HashMap<String, Arc<[ResolvedDep]>>>,
    /// Cache metrics (hits, misses, total lookups)
    pub(super) cache_hits: std::sync::atomic::AtomicU64,
    pub(super) cache_misses: std::sync::atomic::AtomicU64,
}

impl std::fmt::Debug for Resolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Debug must never panic: poisoned memos render as placeholders.
        let dedupe_pref = self
            .dedupe_pref
            .read()
            .map(|guard| format!("{:?}", *guard))
            .unwrap_or_else(|_| "<poisoned>".to_string());
        let existing_versions = self
            .existing_versions
            .read()
            .map(|guard| format!("{} entries", guard.len()))
            .unwrap_or_else(|_| "<poisoned>".to_string());
        f.debug_struct("Resolver")
            .field("overrides", &self.overrides)
            .field("dedupe_pref", &dedupe_pref)
            .field("existing_versions", &existing_versions)
            .finish()
    }
}

impl Resolver {
    pub fn new(provider: std::sync::Arc<dyn DependencyProvider>) -> Self {
        Self {
            provider,
            overrides: HashMap::new(),
            dedupe_pref: std::sync::RwLock::new(DedupePref::default()),
            existing_versions: std::sync::RwLock::new(HashMap::new()),
            dep_memo_cache: std::sync::RwLock::new(HashMap::new()),
            cache_hits: std::sync::atomic::AtomicU64::new(0),
            cache_misses: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn set_overrides(&mut self, overrides: HashMap<String, String>) {
        self.overrides = overrides;
    }

    /// Set dedupe preference (02 §2.1). Default is PreferLatest (safe).
    /// Poisoned lock keeps the previous preference instead of panicking.
    pub fn set_dedupe_pref(&self, pref: DedupePref) {
        if let Ok(mut guard) = self.dedupe_pref.write() {
            *guard = pref;
        }
    }

    /// Provide versions already present in the project (from lockfile) so
    /// PreferExisting can reuse them instead of installing new instances.
    /// Poisoned lock keeps the previous table instead of panicking.
    pub fn set_existing_versions(&self, existing: HashMap<String, Version>) {
        if let Ok(mut guard) = self.existing_versions.write() {
            *guard = existing;
        }
    }

    /// Get dependency memoization cache statistics.
    /// Returns (hits, misses, hit_rate_percent).
    pub fn cache_stats(&self) -> (u64, u64, f64) {
        let hits = self.cache_hits.load(std::sync::atomic::Ordering::Relaxed);
        let misses = self.cache_misses.load(std::sync::atomic::Ordering::Relaxed);
        let total = hits + misses;
        let hit_rate = if total > 0 {
            (hits as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        (hits, misses, hit_rate)
    }
}

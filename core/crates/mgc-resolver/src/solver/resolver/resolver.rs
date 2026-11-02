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
    /// SEC-05: Registry metadata for dependency confusion protection.
    /// Key: "name@version", Value: registry host (e.g. "registry.npmjs.org")
    /// Hybrid approach: side-channel metadata until v1.3.0 data model refactor.
    pub(super) registry_metadata: std::sync::RwLock<HashMap<String, String>>,
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
            registry_metadata: std::sync::RwLock::new(HashMap::new()),
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

    /// Record registry metadata for a resolved package (SEC-05 dependency confusion).
    /// Hybrid approach: side-channel metadata tracking until v1.3.0 data model refactor.
    /// (Ghi metadata registry cho package đã resolve — SEC-05 bảo vệ dependency confusion.)
    pub(super) fn record_registry_metadata(&self, name: &str, version: &Version) {
        if let Some(registry_host) = self.provider.get_registry_host() {
            let key = format!("{}@{}", name, version);
            if let Ok(mut guard) = self.registry_metadata.write() {
                guard.insert(key, registry_host);
            }
        }
    }

    /// Check for dependency confusion attacks using hybrid metadata approach (SEC-05).
    /// Converts Resolution + registry_metadata to DepInfo, then calls check_dependency_confusion.
    /// (Kiểm tra tấn công dependency confusion bằng hybrid metadata — SEC-05.)
    pub(super) fn check_dependency_confusion_hybrid(
        &self,
        resolutions: &[super::Resolution],
    ) -> Result<Vec<String>, super::SolveError> {
        use crate::solver::dependency_confusion::{DepInfo, check_dependency_confusion};
        use std::collections::HashMap;

        // Extract registry metadata snapshot.
        let registry_snapshot = self
            .registry_metadata
            .read()
            .map(|guard| guard.clone())
            .unwrap_or_default();

        // Convert resolutions to DepInfo.
        let dependencies: Vec<DepInfo> = resolutions
            .iter()
            .map(|res| {
                let name = res.package_id.name().as_str().to_string();
                let version = Some(res.version.to_string());
                let key = format!("{}@{}", name, res.version);
                let registry = registry_snapshot.get(&key).cloned();
                DepInfo {
                    name,
                    version,
                    registry,
                }
            })
            .collect();

        // TODO: Implement workspace package detection (read from manifest/lockfile).
        // For now, empty list (no workspace packages to check against).
        let workspace_packages: Vec<String> = Vec::new();

        // TODO: Implement scoped registry mapping (read from .magicore.toml / .npmrc).
        // For now, empty map (no scoped registry validation).
        let scoped_registries: HashMap<String, String> = HashMap::new();

        // TODO: Implement trusted registry list (read from config).
        // For now, empty list (all registries trusted).
        let trusted_registries: Vec<String> = Vec::new();

        // Call the dependency confusion checker.
        let warnings = check_dependency_confusion(
            &workspace_packages,
            &dependencies,
            &scoped_registries,
            &trusted_registries,
        );

        Ok(warnings)
    }
}

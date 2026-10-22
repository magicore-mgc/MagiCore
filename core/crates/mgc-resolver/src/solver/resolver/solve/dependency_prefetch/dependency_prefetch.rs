// Dependency prefetch — Tải dependency và quản lý memo cache.
// Preserve cache metrics and provider-hook ordering — Giữ nguyên metric cache và thứ tự provider hook.
use crate::solver::provider::ResolvedDep;
use crate::solver::resolver::solve::types::{PrefetchedDependencies, SelectedPackage};
use crate::solver::resolver::{Resolver, SolveError, profile::ResolverProfile};
use mgc_types::{PackageId, PackageName, Version};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

impl Resolver {
    pub(in crate::solver::resolver::solve) async fn prefetch_selected_dependencies(
        &self,
        selected: &[SelectedPackage],
        prefetched_versions: &HashMap<String, Arc<[Version]>>,
        profile: &ResolverProfile,
    ) -> Result<(PrefetchedDependencies, Vec<PackageName>), SolveError> {
        let deps_prefetch_started_at = Instant::now();
        let ids: Vec<PackageId> = selected
            .iter()
            .map(|(name, _, version)| PackageId::new(name.clone(), version.clone()))
            .collect();

        // G2: Dependency memoization — check cache before prefetch; kiểm tra memo cache trước khi tải
        let mut dependency_results = HashMap::new();
        let mut uncached_ids = Vec::new();
        {
            let cache = self.dep_memo_cache.read().map_err(|_| SolveError {
                message: "dependency memo cache lock poisoned".to_string(),
            })?;
            for id in &ids {
                let key = format!("{}@{}", id.name_str(), id.version());
                if let Some(cached_deps) = cache.get(&key) {
                    self.cache_hits
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    dependency_results.insert(id.clone(), cached_deps.to_vec());
                } else {
                    self.cache_misses
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    uncached_ids.push(id.clone());
                }
            }
        }

        // Fetch uncached dependencies — Tải dependency chưa có trong cache
        if !uncached_ids.is_empty() {
            let fetched = self
                .provider
                .prefetch_dependencies(&uncached_ids)
                .await
                .map_err(|e| SolveError {
                    message: format!("dependency prefetch failed: {e}"),
                })?;

            // Store in cache — Lưu dependency vào cache
            let mut cache = self.dep_memo_cache.write().map_err(|_| SolveError {
                message: "dependency memo cache lock poisoned".to_string(),
            })?;
            for (id, deps) in fetched {
                let key = format!("{}@{}", id.name_str(), id.version());
                let deps_arc = Arc::<[ResolvedDep]>::from(deps.clone());
                cache.insert(key, deps_arc.clone());
                dependency_results.insert(id, deps);
            }
        }

        profile.mark("prefetch_dependencies", deps_prefetch_started_at);
        self.provider
            .on_batch_resolved(&ids)
            .await
            .map_err(|e| SolveError {
                message: format!("on_batch_resolved hook failed: {e}"),
            })?;
        let prefetched_dependencies: HashMap<PackageId, Arc<[ResolvedDep]>> = dependency_results
            .into_iter()
            .map(|(id, deps)| (id, Arc::<[ResolvedDep]>::from(deps)))
            .collect();
        let mut next_prefetch_seen = HashSet::new();
        let next_prefetch_names: Vec<PackageName> = prefetched_dependencies
            .values()
            .flat_map(|deps| deps.iter())
            .filter_map(|dep| {
                let name = dep.package.as_str().to_string();
                if prefetched_versions.contains_key(&name) || !next_prefetch_seen.insert(name) {
                    None
                } else {
                    Some(dep.package.clone())
                }
            })
            .collect();
        Ok((prefetched_dependencies, next_prefetch_names))
    }
}

//! `prefetch_dependencies.rs` — prefetch dependencies.
//! Prefetch dependency edge.

use super::super::provider::NpmDependencyProvider;
use mgc_resolver::DependencyError;
use mgc_resolver::ResolvedDep;
use mgc_types::{PackageId, PackageName};

impl NpmDependencyProvider {
    pub(in crate::provider) async fn prefetch_dependencies_impl(
        &self,
        ids: &[PackageId],
    ) -> Result<Vec<(PackageId, Vec<ResolvedDep>)>, DependencyError> {
        let mut results = Vec::with_capacity(ids.len());
        let mut preloaded = Vec::new();

        for id in ids {
            let cache_key = Self::version_key(id);
            if let Some(cached) = self.registry_cache.get_deps(&cache_key) {
                results.push((id.clone(), cached));
            } else if let Some(meta) = self
                .metadata_cache
                .get(self.source_package_name(id.name()).as_str())
            {
                let deps = meta
                    .versions
                    .get(&id.version().to_string())
                    .map(|version| self.collect_version_dependencies(version))
                    .unwrap_or_default();
                self.registry_cache.insert_deps(cache_key, deps.clone());
                results.push((id.clone(), deps));
            } else {
                preloaded.push(id.clone());
            }
        }

        if preloaded.is_empty() {
            return Ok(results);
        }

        let missing_names: Vec<PackageName> =
            preloaded.iter().map(|id| id.name().clone()).collect();
        let fetched_metadata = self.prefetch_resolution_metadata(&missing_names).await?;

        for package_id in preloaded {
            // Results are keyed by ALIAS (the name edges request); fall
            // back to the source name for unaliased packages. Looking up
            // by source only broke every real npm: alias
            // (e.g. `vite → @voidzero-dev/vite-plus-core` in Nuxt's tree).
            // (Kết quả key theo ALIAS; fallback tên gốc.)
            let source_name = self.source_package_name(package_id.name());
            let Some(meta) = fetched_metadata
                .get(package_id.name_str())
                .or_else(|| fetched_metadata.get(source_name.as_str()))
            else {
                return Err(DependencyError(format!(
                    "prefetch metadata missing result for '{}'",
                    package_id.name_str()
                )));
            };
            let deps = meta
                .versions
                .get(&package_id.version().to_string())
                .map(|version| self.collect_version_dependencies(version))
                .unwrap_or_default();
            self.registry_cache
                .insert_deps(Self::version_key(&package_id), deps.clone());
            results.push((package_id, deps));
        }

        Ok(results)
    }
}

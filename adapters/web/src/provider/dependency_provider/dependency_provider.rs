//! `dependency_provider.rs` — dependency provider.
//! Implementation trait resolver.

use super::super::provider::NpmDependencyProvider;
use async_trait::async_trait;
use mgc_resolver::{DependencyError, DependencyProvider, ResolvedDep};
use mgc_types::{PackageId, PackageName, Version};

#[async_trait]
impl DependencyProvider for NpmDependencyProvider {
    async fn get_versions(&self, package: &PackageName) -> Result<Vec<Version>, DependencyError> {
        if let Some(cached) = self.cached_versions_for(package) {
            return Ok(cached);
        }
        let meta = self.metadata(package).await?;
        let v = self.versions_with_age_gate(package, &meta)?;
        self.insert_versions_for(package, v.clone());
        Ok(v)
    }

    async fn get_dependencies(
        &self,
        package_id: &PackageId,
    ) -> Result<Vec<ResolvedDep>, DependencyError> {
        let cache_key = Self::version_key(package_id);
        if let Some(cached) = self.registry_cache.get_deps(&cache_key) {
            return Ok(cached);
        }
        let meta = self.metadata(package_id.name()).await?;
        let deps: Vec<ResolvedDep> = meta
            .versions
            .get(&package_id.version().to_string())
            .map(|version| self.collect_version_dependencies(version))
            .unwrap_or_default();
        self.registry_cache.insert_deps(cache_key, deps.clone());
        Ok(deps)
    }

    async fn should_enqueue(&self, dep: &ResolvedDep) -> Result<bool, DependencyError> {
        NpmDependencyProvider::should_enqueue_impl(self, dep).await
    }

    async fn prefetch_versions(
        &self,
        packages: &[PackageName],
    ) -> Result<Vec<(PackageName, Vec<Version>)>, DependencyError> {
        NpmDependencyProvider::prefetch_versions_impl(self, packages).await
    }

    async fn prefetch_dependencies(
        &self,
        ids: &[PackageId],
    ) -> Result<Vec<(PackageId, Vec<ResolvedDep>)>, DependencyError> {
        NpmDependencyProvider::prefetch_dependencies_impl(self, ids).await
    }

    async fn on_batch_resolved(&self, _ids: &[PackageId]) -> Result<(), DependencyError> {
        Ok(())
    }
}

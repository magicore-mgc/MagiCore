use async_trait::async_trait;
use futures_util::future::join_all;
use mgc_types::{PackageId, PackageName, Version};

use super::{DependencyError, ResolvedDep};

/// Dependency provider trait — each ecosystem must implement this.
///
/// WARN: Both methods return `Result`. On network failure, error propagates.
///    Empty vec is valid (package exists with no versions / no deps).
#[async_trait]
pub trait DependencyProvider: Send + Sync {
    async fn get_versions(&self, package: &PackageName) -> Result<Vec<Version>, DependencyError>;
    async fn get_dependencies(&self, id: &PackageId) -> Result<Vec<ResolvedDep>, DependencyError>;
    async fn should_enqueue(&self, dep: &ResolvedDep) -> Result<bool, DependencyError> {
        let _ = dep;
        Ok(true)
    }

    async fn prefetch_versions(
        &self,
        packages: &[PackageName],
    ) -> Result<Vec<(PackageName, Vec<Version>)>, DependencyError> {
        if packages.is_empty() {
            return Ok(Vec::new());
        }
        // Fire all version lookups concurrently — eliminates the N×latency serial bottleneck.
        let futures: Vec<_> = packages
            .iter()
            .map(|name| {
                let name = name.clone();
                async move {
                    let versions = self.get_versions(&name).await?;
                    Ok::<_, DependencyError>((name, versions))
                }
            })
            .collect();
        let results = join_all(futures).await;
        results.into_iter().collect()
    }

    async fn prefetch_dependencies(
        &self,
        ids: &[PackageId],
    ) -> Result<Vec<(PackageId, Vec<ResolvedDep>)>, DependencyError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        // Fire all dependency fetches concurrently.
        let futures: Vec<_> = ids
            .iter()
            .map(|id| {
                let id = id.clone();
                async move {
                    let deps = self.get_dependencies(&id).await?;
                    Ok::<_, DependencyError>((id, deps))
                }
            })
            .collect();
        let results = join_all(futures).await;
        results.into_iter().collect()
    }

    /// Called by solver after each batch resolves exact package versions.
    /// Provider can use this to start background work (e.g. tarball download).
    async fn on_batch_resolved(&self, packages: &[PackageId]) -> Result<(), DependencyError> {
        let _ = packages;
        Ok(())
    }
}

// Legacy adapter forwarding and fail-closed unsupported capability implementations.
// Chuyển tiếp adapter cũ và từ chối capability chưa hỗ trợ theo fail-closed.
//! Bridges legacy adapters and fail-closed unsupported capabilities.
//! Nối adapter cũ và từ chối fail-closed capability chưa hỗ trợ.
use crate::{CacheBackend, Fetcher, Linker, Resolver, TemplateGenerator};
use async_trait::async_trait;
use mgc_types::Ecosystem;
use mgc_types::adapter::{InstallOptions, InstallSummary, PackageAdapter, ResolvedGraph};
use mgc_types::error::MgResult;
use mgc_types::manifest::Manifest;
use std::path::Path;
use std::sync::Arc;

pub(in crate::plugin) struct ResolverBridge(pub(in crate::plugin) Arc<dyn PackageAdapter>);
#[async_trait]
impl Resolver for ResolverBridge {
    async fn resolve(&self, manifest: &Manifest) -> MgResult<ResolvedGraph> {
        self.0.resolve(manifest).await
    }
}

pub(in crate::plugin) struct FetcherBridge(pub(in crate::plugin) Arc<dyn PackageAdapter>);
#[async_trait]
impl Fetcher for FetcherBridge {
    async fn fetch(&self, graph: &ResolvedGraph) -> MgResult<()> {
        self.0.fetch(graph).await
    }
}

pub(in crate::plugin) struct LinkerBridge(pub(in crate::plugin) Arc<dyn PackageAdapter>);
#[async_trait]
impl Linker for LinkerBridge {
    async fn link(
        &self,
        graph: &ResolvedGraph,
        project_root: &Path,
        opts: InstallOptions,
    ) -> MgResult<InstallSummary> {
        self.0.install(graph, project_root, opts).await
    }
}

pub(in crate::plugin) struct TemplateUnsupported {
    pub ecosystem: Ecosystem,
}
#[async_trait]
impl TemplateGenerator for TemplateUnsupported {
    async fn generate(&self, _: &Path, _: &str) -> MgResult<Manifest> {
        Err(mgc_types::error::MgError::Other(format!(
            "template generator not implemented for '{}'",
            self.ecosystem.as_str()
        )))
    }
}

pub(in crate::plugin) struct CacheUnsupported {
    pub ecosystem: Ecosystem,
}
#[async_trait]
impl CacheBackend for CacheUnsupported {
    async fn get(&self, _: &str) -> MgResult<Option<std::path::PathBuf>> {
        Err(mgc_types::error::MgError::Other(format!(
            "cache backend not implemented for '{}'",
            self.ecosystem.as_str()
        )))
    }
    async fn put(&self, _: &str, _: &[u8]) -> MgResult<()> {
        Err(mgc_types::error::MgError::Other(format!(
            "cache backend not implemented for '{}'",
            self.ecosystem.as_str()
        )))
    }
    async fn claim(&self, _: &std::path::Path, _: &[String]) -> MgResult<()> {
        Err(mgc_types::error::MgError::Other(format!(
            "cache backend not implemented for '{}'",
            self.ecosystem.as_str()
        )))
    }
    async fn release(&self, _: &std::path::Path) -> MgResult<()> {
        Err(mgc_types::error::MgError::Other(format!(
            "cache backend not implemented for '{}'",
            self.ecosystem.as_str()
        )))
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

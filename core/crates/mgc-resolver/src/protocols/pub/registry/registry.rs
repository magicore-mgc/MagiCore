//! Stable RegistryProtocol behavior for the pub.dev engine.
//! Hành vi RegistryProtocol ổn định của engine pub.dev.

use super::super::{
    PubProtocol, graph_resolution::GraphResolutionError, package_catalog::select_entry,
    version_constraints::dart_matches,
};
use crate::protocols::{RegistryProtocol, ResolvedEntry};
use async_trait::async_trait;
use mgc_types::{MgResult, Version};

#[async_trait]
impl RegistryProtocol for PubProtocol {
    fn matches_requirement(&self, range: &str, version: &Version) -> bool {
        dart_matches(range, version)
    }

    async fn resolve(&self, name: &str, range: &str) -> MgResult<ResolvedEntry> {
        let doc = self.package_doc(name).await?;
        select_entry(name, &doc, &[range.to_string()])
    }

    async fn resolve_graph_roots(
        &self,
        roots: &[(String, String)],
    ) -> MgResult<Vec<ResolvedEntry>> {
        self.resolve_joint_roots(roots)
            .await
            .map_err(GraphResolutionError::into_mg_error)
    }

    async fn download(&self, entry: &ResolvedEntry) -> MgResult<Vec<u8>> {
        self.download_entry(entry).await
    }
}

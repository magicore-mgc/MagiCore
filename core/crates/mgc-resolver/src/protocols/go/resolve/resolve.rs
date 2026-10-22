//! Stable Go RegistryProtocol implementation and owner delegation.
//! Cài đặt RegistryProtocol ổn định và ủy quyền tới các owner Go.

use crate::protocols::go::{GoModProtocol, go_matches};
use crate::protocols::{RegistryProtocol, ResolvedEntry};
use async_trait::async_trait;
use mgc_types::{MgResult, Version};

#[async_trait]
impl RegistryProtocol for GoModProtocol {
    fn matches_requirement(&self, range: &str, version: &Version) -> bool {
        go_matches(range, version)
    }

    async fn resolve(&self, name: &str, range: &str) -> MgResult<ResolvedEntry> {
        self.resolve_by_requirement(name, range).await
    }

    /// Resolve all project roots together using Go's minimum version selection.
    /// Chọn version tối thiểu đáp ứng mọi requirement trong graph Go nhiều root.
    async fn resolve_graph_roots(
        &self,
        roots: &[(String, String)],
    ) -> MgResult<Vec<ResolvedEntry>> {
        self.resolve_mvs_roots(roots).await
    }

    async fn resolve_graph(&self, name: &str, range: &str) -> MgResult<Vec<ResolvedEntry>> {
        self.resolve_graph_roots(&[(name.to_string(), range.to_string())])
            .await
    }

    async fn download(&self, entry: &ResolvedEntry) -> MgResult<Vec<u8>> {
        self.download_artifact(entry).await
    }

    fn verify(&self, entry: &ResolvedEntry, bytes: &[u8]) -> MgResult<()> {
        self.verify_artifact(entry, bytes)
    }
}

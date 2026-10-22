//! Artifact fetch capability implementation.
//! Triển khai capability tải artifact.

use super::super::{CoreIdent, unsupported_capability};
use crate::adapter::ResolvedGraph;
use crate::error::MgResult;
use async_trait::async_trait;

/// Downloads resolved artifacts (registry tarballs / modules).
/// Tải artifact đã resolve (tarball registry / module).
#[async_trait]
pub trait ArtifactFetcher: CoreIdent {
    fn probe_artifact_fetcher(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "fetch",
            "nothing for mgc to fetch; the core's toolchain owns downloads",
        ))
    }

    async fn fetch(&self, _graph: &ResolvedGraph) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "fetch",
            "nothing for mgc to fetch; the core's toolchain owns downloads",
        ))
    }
}

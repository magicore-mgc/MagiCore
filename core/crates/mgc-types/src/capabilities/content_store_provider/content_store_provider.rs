//! Content store capability implementation.
//! Triển khai capability content store.

use super::super::{CoreIdent, unsupported_capability};
use crate::adapter::{InstallOptions, InstallSummary, ResolvedGraph};
use crate::error::MgResult;
use async_trait::async_trait;
use std::path::Path;

/// Installs the resolved graph (mgc content-store backed where available).
/// Cài graph đã resolve (qua content-store của mgc khi có).
#[async_trait]
pub trait ContentStoreProvider: CoreIdent {
    fn probe_content_store(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "install",
            "this core has no mgc registry install; dependencies are materialized by the core's own workflow",
        ))
    }

    async fn install(
        &self,
        _graph: &ResolvedGraph,
        _project_root: &Path,
        _opts: InstallOptions,
    ) -> MgResult<InstallSummary> {
        Err(unsupported_capability(
            self.core_id(),
            "install",
            "this core has no mgc registry install; dependencies are materialized by the core's own workflow",
        ))
    }
}

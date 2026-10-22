//! Keeps CDK/Pulumi install on the embedded Web engine; Terraform never falls back to the Terraform CLI as a package manager. — Giữ install CDK/Pulumi qua Web engine nhúng; Terraform không fallback sang Terraform CLI để quản lý package.

use async_trait::async_trait;
use std::path::Path;

use mgc_types::adapter::{InstallOptions, InstallSummary};
use mgc_types::capabilities::ContentStoreProvider;
use mgc_types::{MgResult, ResolvedGraph};

use super::super::CloudAdapter;

#[async_trait]
impl ContentStoreProvider for CloudAdapter {
    fn probe_content_store(&self) -> MgResult<()> {
        if self.web.is_some() {
            Ok(())
        } else {
            Err(mgc_types::capabilities::unsupported_capability(
                "cloud",
                "content_store",
                "Terraform install delegates to the terraform CLI and is not an MGC content store",
            ))
        }
    }

    async fn install(
        &self,
        graph: &ResolvedGraph,
        project_root: &Path,
        opts: InstallOptions,
    ) -> MgResult<InstallSummary> {
        if let Some(web) = &self.web {
            return web.install(graph, project_root, opts).await;
        }
        Err(mgc_types::capabilities::unsupported_capability(
            "cloud",
            "install",
            "Terraform provider initialization is unsupported by MagiCore; no external package manager is invoked by PackageAdapter::install",
        ))
    }
}

//! Forwards content-store installation and probe operations to the selected native Python lane.
//! Chuyển tiếp thao tác cài vào content store và probe tới lane Python native được chọn.

use crate::adapter::AiAdapter;
use async_trait::async_trait;
use mgc_types::adapter::{InstallOptions, InstallSummary};
use mgc_types::{MgResult, ResolvedGraph};
use std::path::Path;

#[async_trait]
impl mgc_types::capabilities::ContentStoreProvider for AiAdapter {
    fn probe_content_store(&self) -> MgResult<()> {
        match &self.python_lane {
            Some(py) => py.probe_content_store(),
            None => Err(mgc_types::capabilities::unsupported_capability(
                "ai",
                "install",
                "no pyproject.toml — native PyPI install requires an MGC-owned pyproject; foreign lockfiles are unsupported",
            )),
        }
    }

    async fn install(
        &self,
        graph: &ResolvedGraph,
        project_root: &Path,
        opts: InstallOptions,
    ) -> MgResult<InstallSummary> {
        match &self.python_lane {
            Some(py) => py.install(graph, project_root, opts).await,
            None => Err(mgc_types::capabilities::unsupported_capability(
                "ai",
                "install",
                "no pyproject.toml — native PyPI install requires an MGC-owned pyproject; foreign lockfiles are unsupported",
            )),
        }
    }
}

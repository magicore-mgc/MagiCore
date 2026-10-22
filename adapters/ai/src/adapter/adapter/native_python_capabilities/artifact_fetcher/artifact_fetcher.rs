//! Forwards verified artifact fetching to the selected native Python lane.
//! Chuyển tiếp fetch artifact đã xác thực tới lane Python native được chọn.

use crate::adapter::AiAdapter;
use async_trait::async_trait;
use mgc_types::{MgResult, ResolvedGraph};

#[async_trait]
impl mgc_types::capabilities::ArtifactFetcher for AiAdapter {
    fn probe_artifact_fetcher(&self) -> MgResult<()> {
        match &self.python_lane {
            Some(py) => py.probe_artifact_fetcher(),
            None => Err(mgc_types::capabilities::unsupported_capability(
                "ai",
                "fetch",
                "no pyproject.toml — nothing for the native PyPI engine to fetch",
            )),
        }
    }

    async fn fetch(&self, graph: &ResolvedGraph) -> MgResult<()> {
        match &self.python_lane {
            Some(py) => py.fetch(graph).await,
            None => Err(mgc_types::capabilities::unsupported_capability(
                "ai",
                "fetch",
                "no pyproject.toml — nothing for the native PyPI engine to fetch",
            )),
        }
    }
}

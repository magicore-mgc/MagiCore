use super::super::GameAdapter;
use async_trait::async_trait;
use mgc_types::adapter::{InstallOptions, InstallSummary};
use mgc_types::capabilities::ContentStoreProvider;
use mgc_types::{MgResult, ResolvedGraph};
use std::path::Path;

#[async_trait]
impl ContentStoreProvider for GameAdapter {
    fn probe_content_store(&self) -> MgResult<()> {
        Err(mgc_types::capabilities::unsupported_capability(
            "game",
            "content_store",
            "Bevy uses the shared native Lib/Rust engine; the GameAdapter does not own a store",
        ))
    }

    async fn install(
        &self,
        _graph: &ResolvedGraph,
        _project_root: &Path,
        _opts: InstallOptions,
    ) -> MgResult<InstallSummary> {
        Err(mgc_types::capabilities::unsupported_capability(
            "game",
            "install",
            "the GameAdapter has no MagiCore-owned installer for this ecosystem; dependency installation is unsupported",
        ))
    }
}

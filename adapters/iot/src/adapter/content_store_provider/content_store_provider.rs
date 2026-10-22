//! Keeps package installation/content-store probes fail-closed; no provider CLI is started. — Giữ probe install/content-store fail-closed; không chạy CLI provider.

use std::path::Path;

use super::super::IotAdapter;
use async_trait::async_trait;
use mgc_types::adapter::{InstallOptions, InstallSummary};
use mgc_types::capabilities::ContentStoreProvider;
use mgc_types::{MgResult, ResolvedGraph};
#[async_trait]
impl ContentStoreProvider for IotAdapter {
    fn probe_content_store(&self) -> MgResult<()> {
        Err(mgc_types::capabilities::unsupported_capability(
            "iot",
            "content_store",
            "esp32-rust uses the shared native Lib/Rust engine; PlatformIO/Zephyr remain toolchain-owned",
        ))
    }

    async fn install(
        &self,
        _graph: &ResolvedGraph,
        _project_root: &Path,
        _opts: InstallOptions,
    ) -> MgResult<InstallSummary> {
        Err(mgc_types::capabilities::unsupported_capability(
            "iot",
            "install",
            "the IoT adapter has no MagiCore-owned installer for this ecosystem; dependency installation is unsupported",
        ))
    }
}

//! `content_store.rs` — Content-store install bridge / cầu nối cài qua content store.
use super::super::WebAdapter;
use crate::install::run_install_with_registry;
use async_trait::async_trait;
use mgc_types::{
    MgResult,
    adapter::{InstallOptions, InstallSummary, ResolvedGraph},
    capabilities::ContentStoreProvider,
};
use std::path::Path;

#[async_trait]
impl ContentStoreProvider for WebAdapter {
    /// Evidence: the CAS-backed install pipeline (install/run_install —
    /// virtual store + content-addressable store).
    /// Dẫn chứng: pipeline install qua CAS (install/run_install — virtual
    /// store + content-addressable store).
    fn probe_content_store(&self) -> MgResult<()> {
        Ok(())
    }

    async fn install(
        &self,
        graph: &ResolvedGraph,
        project_root: &Path,
        opts: InstallOptions,
    ) -> MgResult<InstallSummary> {
        // Phase tracing for CI diagnosability — tường minh điểm vào install
        // (stderr) khi một io error trần làm CI fail không rõ phase.
        eprintln!(
            "[magicore:debug] install:project_root={}",
            project_root.display()
        );
        let prefetch_handle = self.prefetch_handle_guard().take();
        run_install_with_registry(
            &self.provider.registry,
            self.store.as_ref(),
            self.shared_cache.clone(),
            prefetch_handle,
            graph,
            project_root,
            opts,
        )
        .await
    }
}

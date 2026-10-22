//! `artifact_fetch.rs` — Registry artifact fetch / tải artifact từ registry.
use super::super::WebAdapter;
use crate::install;
use async_trait::async_trait;
use mgc_types::{MgResult, adapter::ResolvedGraph, capabilities::ArtifactFetcher};

#[async_trait]
impl ArtifactFetcher for WebAdapter {
    /// Evidence: registry tarball downloader (fetch body below, feeding
    /// the content store; install/ also prefetches via spawn_tarball_download).
    /// Dẫn chứng: bộ tải tarball registry (thân fetch bên dưới, nạp vào
    /// content store; install/ còn prefetch qua spawn_tarball_download).
    fn probe_artifact_fetcher(&self) -> MgResult<()> {
        Ok(())
    }

    async fn fetch(&self, graph: &ResolvedGraph) -> MgResult<()> {
        let reg = &self.provider.registry;
        for pkg in &graph.packages {
            let url = install::fetch::package_tarball_url(reg.registry_url(), pkg);
            let bytes = reg.download_tarball(&url).await.map_err(|e| {
                mgc_types::MgError::Network(format!(
                    "download failed for '{}': {}",
                    pkg.id.name_str(),
                    e
                ))
            })?;
            // Verify declared SRI before publishing any artifact bytes to CAS.
            // Hash nội dung CAS không thay cho SRI của artifact đã resolve.
            install::integrity::verify_tarball_integrity(pkg, &bytes)?;
            if let Some(ref store) = self.store {
                store
                    .import_bytes(&bytes)
                    .map_err(|e| mgc_types::MgError::Store(e.to_string()))?;
            }
        }
        Ok(())
    }
}

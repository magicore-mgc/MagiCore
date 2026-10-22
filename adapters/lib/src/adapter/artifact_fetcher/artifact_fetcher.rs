use crate::adapter::types::LibAdapter;
use async_trait::async_trait;
use mgc_types::capabilities::ArtifactFetcher;
use mgc_types::{MgResult, ResolvedGraph};

#[async_trait]
impl ArtifactFetcher for LibAdapter {
    /// Evidence: delegated fetch — TS rides the web engine (mgc fetcher +
    /// CAS); non-TS lanes FAIL CLOSED (P0-B): a bare Ok(()) claimed
    /// "artifacts fetched" while nothing was downloaded.
    /// Dẫn chứng: fetch ủy quyền — TS đi trên web engine (mgc fetcher +
    /// CAS); lane non-TS FAIL-CLOSED (P0-B): Ok(()) trần tuyên bố "đã
    /// fetch" trong khi không tải gì cả.
    fn probe_artifact_fetcher(&self) -> MgResult<()> {
        if let Some(web) = &self.web {
            return web.probe_artifact_fetcher();
        }
        Err(mgc_types::capabilities::unsupported_capability(
            "lib",
            "fetch",
            "this native language downloads and verifies artifacts inside install; standalone fetch is not exposed",
        ))
    }

    async fn fetch(&self, graph: &ResolvedGraph) -> MgResult<()> {
        if let Some(web) = &self.web {
            return web.fetch(graph).await;
        }
        // P0-B (2026-09-16) fail-closed: delegated fetch must never
        // answer a fake Ok. The toolchain fetches during its own
        // install (managed store); mgc-native fetch lands with the
        // native engine (Phase 2/3).
        // P0-B (2026-09-16) fail-closed: fetch ủy quyền không bao giờ
        // được trả Ok giả. Toolchain tự fetch trong install của nó
        // (store có quản lý); fetch mgc-native thuộc native engine
        // (Phase 2/3).
        Err(mgc_types::MgError::Unsupported {
            core: "lib",
            capability: "fetch",
            guidance: format!(
                "{} artifacts are fetched by the toolchain itself during its \
                 install; mgc-native fetch lands with the native engine \
                 (Phase 2/3)",
                self.language()
            ),
        })
    }
}

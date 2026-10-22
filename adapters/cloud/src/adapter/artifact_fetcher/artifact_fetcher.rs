//! Preserves Web graph fetching and the existing unsupported result for provider modules. — Giữ tải graph qua Web và kết quả unsupported hiện tại cho module provider.

use async_trait::async_trait;

use mgc_types::capabilities::ArtifactFetcher;
use mgc_types::{MgResult, ResolvedGraph};

use super::super::CloudAdapter;

#[async_trait]
impl ArtifactFetcher for CloudAdapter {
    async fn fetch(&self, graph: &ResolvedGraph) -> MgResult<()> {
        if let Some(web) = &self.web {
            return web.fetch(graph).await;
        }
        Err(mgc_types::MgError::Unsupported {
            core: "cloud",
            capability: "fetch",
            guidance: "terraform modules are fetched by `terraform init` \
                       during install; there is no separate fetch step"
                .to_string(),
        })
    }
}

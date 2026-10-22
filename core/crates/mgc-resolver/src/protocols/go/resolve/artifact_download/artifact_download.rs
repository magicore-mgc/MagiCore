//! Fetch the resolved Go module zip bytes.
//! Tải bytes zip Go module đã resolve.

use crate::protocols::ResolvedEntry;
use crate::protocols::go::GoModProtocol;
use mgc_types::MgResult;

impl GoModProtocol {
    /// Download the artifact through the configured Go proxy client.
    /// Tải artifact bằng HTTP client của Go proxy đã cấu hình.
    pub(in crate::protocols::go::resolve) async fn download_artifact(
        &self,
        entry: &ResolvedEntry,
    ) -> MgResult<Vec<u8>> {
        self.get_bytes(&entry.artifact_url).await
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

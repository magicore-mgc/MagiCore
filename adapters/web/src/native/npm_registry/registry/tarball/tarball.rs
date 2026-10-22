//! registry tarball.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

use crate::native::npm_registry::{NpmRegistry, with_retry};
use anyhow::Result;

impl NpmRegistry {
    /// Classic buffered download — kept for compatibility with callers that need
    /// the raw bytes (e.g., integrity verification before store import).
    pub async fn download_tarball(&self, url: &str) -> Result<Vec<u8>> {
        with_retry("tarball", url, || async {
            let resp = self
                .request_for(url, false)?
                .send()
                .await?
                .error_for_status()?;
            let bytes = resp.bytes().await?;
            Ok(bytes.to_vec())
        })
        .await
    }
}

//! registry tarball automatic.
// Chọn đường tải tự động theo kích thước body thực tế, không tin riêng header.

use self::body_stream::download_response_body;
use crate::native::npm_registry::{DownloadedTarball, NpmRegistry, with_retry};
use anyhow::Result;

#[path = "body_stream/mod.rs"]
mod body_stream;

impl NpmRegistry {
    pub async fn download_tarball_auto(
        &self,
        url: &str,
        dest: &std::path::Path,
    ) -> Result<DownloadedTarball> {
        with_retry("tarball-auto", url, || async {
            if let Some(parent) = dest.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }

            let resp = self
                .request_for(url, true)?
                .send()
                .await?
                .error_for_status()?;

            let content_length = resp.content_length();
            download_response_body(resp.bytes_stream(), content_length, dest)
                .await
                .map_err(anyhow::Error::from)
        })
        .await
    }
}

/// Threshold in bytes above which `download_tarball_to_file` is used instead of
/// buffered `download_tarball`. Set to 2 MiB.
pub const LARGE_PKG_THRESHOLD_BYTES: u64 = 2 * 1024 * 1024;

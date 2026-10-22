//! batch tarball.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

use crate::native::npm_registry::{batch_http_client, with_retry};
use anyhow::Result;

pub async fn batch_download_tarball(url: &str) -> Result<Vec<u8>> {
    let client = batch_http_client().map_err(|error| anyhow::anyhow!("{error}"))?;
    with_retry("batch-tarball", url, || async {
        let resp = client.get(url).send().await?.error_for_status()?;
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    })
    .await
}

/// Batch download kèm auth token (registry private)
pub async fn batch_download_tarball_with_auth(url: &str, token: Option<&str>) -> Result<Vec<u8>> {
    let client = batch_http_client().map_err(|error| anyhow::anyhow!("{error}"))?;
    with_retry("batch-tarball", url, || async {
        let mut req = client.get(url);
        if let Some(t) = token {
            req = req.header("Authorization", format!("Bearer {t}"));
        }
        let resp = req.send().await?.error_for_status()?;
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    })
    .await
}

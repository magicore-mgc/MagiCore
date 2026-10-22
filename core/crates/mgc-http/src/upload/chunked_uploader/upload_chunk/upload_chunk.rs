// Upload one chunk — PATCH with the current byte range.
// Gửi một chunk — PATCH kèm byte range hiện tại.

use super::super::ChunkedUploader;
use super::error::ChunkUploadError;
use anyhow::Result;
use reqwest::header::{CONTENT_RANGE, HeaderMap, HeaderValue};

pub(super) fn content_range_value(offset: u64, data_len: usize) -> Result<String> {
    let end = offset
        .checked_add(u64::try_from(data_len)?)
        .and_then(|next| next.checked_sub(1))
        .ok_or_else(|| anyhow::Error::new(ChunkUploadError::OffsetOverflow))?;
    Ok(format!("{offset}-{end}"))
}

impl ChunkedUploader {
    /// Upload a chunk and return the next offset — gửi chunk và trả offset kế tiếp.
    pub async fn upload_chunk(
        &self,
        repo: &str,
        upload_id: &str,
        offset: u64,
        data: &[u8],
    ) -> Result<u64> {
        if data.is_empty() {
            return Ok(offset);
        }
        let url = format!(
            "{}/v2/{}/blobs/uploads/{}",
            self.base_url.trim_end_matches('/'),
            repo,
            upload_id
        );
        let range = content_range_value(offset, data.len())?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_RANGE, HeaderValue::from_str(&range)?);

        let resp = self
            .client
            .patch_with_timeout_and_headers(&url, data.to_vec(), self.timeout_per_chunk, headers)
            .await?;

        if resp.status().as_u16() == 308 {
            // Resume-incomplete response may provide the current server range.
            // תגובת Resume-Incomplete עשויה לספק את טווח השרת הנוכחי.
            let range_header = resp
                .headers()
                .get("range")
                .and_then(|header| header.to_str().ok())
                .unwrap_or("bytes=0-");
            let next_offset = range_header
                .strip_prefix("bytes=0-")
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(offset + data.len() as u64);
            return Ok(next_offset);
        }

        if !resp.status().is_success() {
            return Err(anyhow::Error::new(ChunkUploadError::Failed(resp.status())));
        }
        Ok(offset + data.len() as u64)
    }
}

#[cfg(test)]
#[path = "test/upload_chunk.rs"]
mod tests;

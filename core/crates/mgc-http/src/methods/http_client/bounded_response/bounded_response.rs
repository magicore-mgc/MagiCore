//! Reads HTTP response bodies under a hard byte limit.
//! Đọc body HTTP với giới hạn byte cứng.

use super::super::http_client::HttpClient;
use super::error::BoundedResponseError;
use anyhow::Result;

impl HttpClient {
    /// GET a response while enforcing a hard byte limit on its body.
    /// Tải GET với giới hạn byte cứng cho response body.
    pub async fn get_bytes_limited(&self, url: &str, max_bytes: usize) -> Result<(u16, Vec<u8>)> {
        let mut response = self.execute(self.client.get(url)).await?;
        let status = response.status().as_u16();
        if response
            .content_length()
            .is_some_and(|length| content_length_exceeds_limit(length, max_bytes))
        {
            return Err(anyhow::Error::new(BoundedResponseError::ExceedsByteLimit));
        }
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            append_response_chunk(&mut body, &chunk, max_bytes)?;
        }
        Ok((status, body))
    }
}

fn append_response_chunk(body: &mut Vec<u8>, chunk: &[u8], max_bytes: usize) -> Result<()> {
    if body.len() > max_bytes || chunk.len() > max_bytes - body.len() {
        return Err(anyhow::Error::new(BoundedResponseError::ExceedsByteLimit));
    }
    let required_len = body.len() + chunk.len();
    reserve_response_capacity(body, required_len, max_bytes)?;
    body.extend_from_slice(chunk);
    Ok(())
}

fn reserve_response_capacity(
    body: &mut Vec<u8>,
    required_len: usize,
    max_bytes: usize,
) -> Result<()> {
    if required_len <= body.capacity() {
        return Ok(());
    }
    // Grow geometrically to avoid a reallocation per transport chunk, but cap
    // the requested target at the caller's logical response limit.
    // (Tăng theo cấp số nhân để tránh reallocate mỗi chunk, nhưng chặn target
    // theo giới hạn response logic do caller truyền.)
    let geometric_target = body.capacity().max(1).saturating_mul(2);
    let target_capacity = required_len.max(geometric_target).min(max_bytes);
    body.try_reserve_exact(target_capacity - body.len())
        .map_err(|source| anyhow::Error::new(BoundedResponseError::AllocationFailed(source)))
}

fn content_length_exceeds_limit(content_length: u64, max_bytes: usize) -> bool {
    match usize::try_from(content_length) {
        Ok(length) => length > max_bytes,
        Err(_) => true,
    }
}

#[cfg(test)]
#[path = "test/bounded_response.rs"]
mod tests;

// ChunkedUploader configuration — shared state for OCI upload operations.
// Cấu hình ChunkedUploader — state dùng chung cho các thao tác upload OCI.

use crate::methods::HttpClient;
use std::time::Duration;

/// Chunked OCI upload engine — engine upload OCI theo chunk.
pub struct ChunkedUploader {
    pub(super) client: HttpClient,
    pub(super) base_url: String,
    pub(super) chunk_size: u64,
    pub(super) timeout_per_chunk: Duration,
}

impl ChunkedUploader {
    pub fn new(client: HttpClient, base_url: impl Into<String>) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            chunk_size: 10 * 1024 * 1024,
            timeout_per_chunk: Duration::from_secs(60),
        }
    }

    pub fn with_chunk_size(mut self, size: u64) -> Self {
        self.chunk_size = size;
        self
    }
}

#[cfg(test)]
#[path = "test/chunked_uploader.rs"]
mod tests;

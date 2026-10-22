//! Tarball payload and fetch-result data contract.
//! Hợp đồng dữ liệu payload và kết quả fetch tarball.

use std::path::PathBuf;
use std::sync::Arc;

pub enum TarballPayload {
    Bytes(Arc<[u8]>),
    CachedPath(PathBuf, u64),
}

impl TarballPayload {
    pub fn len(&self) -> u64 {
        match self {
            Self::Bytes(bytes) => bytes.len() as u64,
            Self::CachedPath(_, len) => *len,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct TarballFetchResult {
    pub payload: TarballPayload,
    pub queue_wait_ms: u64,
    pub io_ms: u64,
    pub persist_to_shared_cache: bool,
}

#[cfg(test)]
#[path = "test/fetch_result.rs"]
mod tests;

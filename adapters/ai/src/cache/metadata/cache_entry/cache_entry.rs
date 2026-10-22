//! Cache-entry identity and snapshot fields — field định danh và snapshot cache.

use crate::cache::metadata::MetadataError;
use mgc_types::{MgError, MgResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

/// Metadata snapshot for one cached model.
/// Snapshot metadata cho một model trong cache.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub model_id: String,
    pub size_bytes: u64,
    pub cached_at: SystemTime,
    pub last_accessed: SystemTime,
    pub checksum: Option<String>,
}

impl CacheEntry {
    /// Build metadata from a filesystem path when no sidecar exists.
    /// Tạo metadata từ path filesystem khi chưa có sidecar.
    pub fn from_path(model_id: &str, path: &PathBuf) -> MgResult<Self> {
        let metadata = std::fs::metadata(path)
            .map_err(|source| MgError::from(MetadataError::io("inspect model path", source)))?;
        let cached_at = metadata.created().unwrap_or_else(|_| SystemTime::now());
        let last_accessed = metadata.accessed().unwrap_or_else(|_| SystemTime::now());

        Ok(CacheEntry {
            model_id: model_id.to_string(),
            size_bytes: metadata.len(),
            cached_at,
            last_accessed,
            checksum: None,
        })
    }
}

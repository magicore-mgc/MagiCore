//! Define the persisted metadata envelope and normalized cache record.
//! Định nghĩa envelope metadata lưu trữ và record cache chuẩn hóa.

use serde::{Deserialize, Serialize};

use crate::native;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedMetadataEnvelope {
    pub fetched_at: u64,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub stale_retry_after: Option<u64>,
    pub metadata: native::npm_registry::PackageMetadata,
}

#[derive(Debug, Clone)]
pub struct CachedMetadataRecord {
    pub fetched_at: u64,
    pub etag: Option<String>,
    pub stale_retry_after: Option<u64>,
    pub metadata: native::npm_registry::PackageMetadata,
}

//! Metadata recorded for one cached package.
//! Metadata được ghi cho một package trong cache.

use serde::{Deserialize, Serialize};

/// Cache entry metadata — Metadata entry cache.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub package_id: String,
    pub integrity: String,
    pub cached_at: String,
    pub size_bytes: u64,
}

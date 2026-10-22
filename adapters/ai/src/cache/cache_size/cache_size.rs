//! Report regular-file logical bytes in an AI cache — báo logical bytes file thường trong cache.

use super::error::CacheSizeError;
use crate::cache::cache_dir;
use crate::cache::filesystem::logical_size::logical_size;
use crate::registry::Registry;
use mgc_types::{MgError, MgResult};
use std::fs;

/// Return the logical bytes stored below a registry cache root.
/// Trả số logical bytes nằm dưới cache root của registry.
pub fn cache_size(registry: &Registry) -> MgResult<u64> {
    let cache = cache_dir(registry)?;
    let metadata = match fs::metadata(&cache) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(source) => return Err(CacheSizeError::io("inspect", source).into()),
    };

    if !metadata.is_dir() {
        return Err(CacheSizeError::RootNotDirectory.into());
    }

    let canonical_root = fs::canonicalize(&cache)
        .map_err(|source| MgError::from(CacheSizeError::io("resolve", source)))?;
    logical_size(&canonical_root).map_err(|error| MgError::from(CacheSizeError::Filesystem(error)))
}

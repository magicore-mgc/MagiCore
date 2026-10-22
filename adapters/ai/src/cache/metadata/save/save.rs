//! Serialize and publish metadata — serialize và publish metadata.

use crate::cache::metadata::{CacheEntry, MetadataError, metadata_path};
use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Atomically save a metadata sidecar beside the model path.
/// Ghi atomic sidecar metadata cạnh model path.
pub fn save_metadata(path: &Path, entry: &CacheEntry) -> MgResult<()> {
    let bytes = serde_json::to_vec_pretty(entry)
        .map_err(|source| MgError::from(MetadataError::Serialize(source)))?;
    let sidecar = metadata_path(path);
    super::super::atomic_publish::publish_metadata(&sidecar, &bytes).map_err(MgError::from)
}

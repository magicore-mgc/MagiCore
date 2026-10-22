//! Reads cached metadata unless the age gate requires a fresh full packument.
//! Đọc metadata cache trừ khi age gate bắt buộc lấy packument mới đầy đủ.

use crate::cache::SharedWebCache;
use crate::cache_metadata::CachedMetadataRecord;
use crate::native::npm_registry::NpmRegistry;
use mgc_resolver::DependencyError;

pub(in crate::cache_metadata) fn load_cached_metadata(
    package: &str,
    registry: &NpmRegistry,
    shared_cache: Option<&SharedWebCache>,
    registry_url: &str,
) -> Result<Option<CachedMetadataRecord>, DependencyError> {
    if registry.age_gate_armed() {
        return Ok(None);
    }

    match shared_cache {
        Some(cache) => cache.read_metadata(package, registry_url),
        None => Ok(None),
    }
}

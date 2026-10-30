//! Reads cached metadata unless the age gate requires a fresh full packument.
//! Đọc metadata cache trừ khi age gate bắt buộc lấy packument mới đầy đủ.

use crate::cache::SharedWebCache;
use crate::cache_metadata::CachedMetadataRecord;
use crate::native::npm_registry::NpmRegistry;
use mgc_resolver::DependencyError;

pub(in crate::cache_metadata) fn load_cached_metadata(
    package: &str,
    _registry: &NpmRegistry,
    shared_cache: Option<&SharedWebCache>,
    registry_url: &str,
) -> Result<Option<CachedMetadataRecord>, DependencyError> {
    let Some(cache) = shared_cache else {
        return Ok(None);
    };
    let cached = cache.read_metadata(package, registry_url)?;

    // Every cached entry is returned here; the age decision is made once, by
    // the resolver guard, on whatever metadata it receives. A packument
    // without `time` cannot clear quarantine there, so an abbreviated cache
    // entry cannot smuggle a young release past the gate, and an entry that
    // carries `time` is reused at full speed.
    // (Mọi entry cache trả về đây; quyết định tuổi do guard của resolver làm
    // trên metadata nhận được. Packument thiếu `time` không qua được
    // quarantine, còn entry có `time` được tái sử dụng đầy đủ.)
    Ok(cached)
}

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
    let Some(cache) = shared_cache else {
        return Ok(None);
    };
    let cached = cache.read_metadata(package, registry_url)?;

    // Under an armed age gate an entry without the `time` map cannot answer
    // the age decision: abbreviated packuments legitimately omit it, and the
    // version filter would then refuse every version of the package. Such an
    // entry must be re-fetched as a full packument instead of being reused.
    // An entry that carries `time` is reused at full speed.
    // (Khi cổng tuổi bật, entry thiếu map `time` không trả lời được quyết định
    // tuổi: packument abbreviated vốn không có map này nên filter version sẽ
    // từ chối mọi version. Entry đó phải fetch lại full packument; entry có
    // `time` vẫn được tái sử dụng đầy đủ.)
    if registry.age_gate_armed() && cached.as_ref().is_some_and(|c| c.metadata.time.is_empty()) {
        return Ok(None);
    }

    Ok(cached)
}

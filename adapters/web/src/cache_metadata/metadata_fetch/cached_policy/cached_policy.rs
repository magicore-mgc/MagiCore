//! Reuses fresh or retry-deferred metadata only under the existing stale-age policy.
//! Chỉ tái sử dụng metadata mới hoặc đang cooldown theo chính sách tuổi stale hiện có.

use crate::cache_metadata::{
    CachedMetadataRecord, metadata_record_is_fresh, metadata_record_is_usable_stale,
    metadata_record_retry_deferred,
};
use crate::native::npm_registry::PackageMetadata;
use std::sync::Arc;

pub(in crate::cache_metadata) fn reuse_without_refresh(
    cached: &CachedMetadataRecord,
) -> Option<Arc<PackageMetadata>> {
    if metadata_record_is_fresh(cached) {
        return Some(Arc::new(cached.metadata.clone()));
    }

    if metadata_record_retry_deferred(cached) && metadata_record_is_usable_stale(cached) {
        return Some(Arc::new(cached.metadata.clone()));
    }

    None
}

//! Refreshes an existing metadata record using its ETag and preserves stale fallback rules.
//! Refresh record metadata hiện có bằng ETag và giữ quy tắc fallback stale.

use crate::cache::SharedWebCache;
use crate::cache_metadata::{
    CachedMetadataRecord, metadata_record_is_usable_stale, next_stale_retry_after,
};
use crate::native::npm_registry::{NpmRegistry, PackageMetadata};
use mgc_resolver::DependencyError;
use std::sync::Arc;

pub(in crate::cache_metadata) async fn refresh_cached_metadata(
    package: &str,
    registry: &NpmRegistry,
    shared_cache: Option<&SharedWebCache>,
    registry_url: &str,
    cached: &CachedMetadataRecord,
) -> Option<Result<Arc<PackageMetadata>, DependencyError>> {
    let etag = cached.etag.as_ref()?;
    let result = registry
        .fetch_metadata_conditional(package, Some(etag))
        .await;

    Some(match result {
        Ok(None) => {
            if let Some(shared_cache) = shared_cache {
                let _ = shared_cache.write_metadata(
                    package,
                    &cached.metadata,
                    Some(etag.clone()),
                    registry_url,
                );
            }
            Ok(Arc::new(cached.metadata.clone()))
        }
        Ok(Some((metadata, new_etag))) => {
            if let Some(shared_cache) = shared_cache {
                let _ =
                    shared_cache.write_metadata(package, &metadata, Some(new_etag), registry_url);
            }
            Ok(Arc::new(metadata))
        }
        Err(_) => {
            if !metadata_record_is_usable_stale(cached) {
                return Some(Err(DependencyError(format!(
                    "npm metadata refresh failed for '{}' and cached metadata is too old to reuse",
                    package
                ))));
            }
            if let Some(shared_cache) = shared_cache {
                let _ = shared_cache.write_metadata_record(
                    package,
                    &cached.metadata,
                    cached.etag.clone(),
                    cached.fetched_at,
                    Some(next_stale_retry_after()),
                    registry_url,
                );
            }
            Ok(Arc::new(cached.metadata.clone()))
        }
    })
}

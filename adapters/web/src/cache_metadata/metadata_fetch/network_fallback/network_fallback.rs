//! Fetches a full registry record and applies the existing bounded stale fallback.
//! Tải record registry đầy đủ và áp dụng fallback stale có giới hạn hiện tại.

use crate::cache::SharedWebCache;
use crate::cache::current_unix_secs;
use crate::cache_metadata::{
    CachedMetadataRecord, metadata_record_is_usable_stale, next_stale_retry_after,
};
use crate::native::npm_registry::{NpmRegistry, PackageMetadata};
use mgc_resolver::DependencyError;
use std::sync::Arc;

pub(in crate::cache_metadata) async fn fetch_with_stale_fallback(
    package: &str,
    registry: &NpmRegistry,
    shared_cache: Option<&SharedWebCache>,
    registry_url: &str,
    cached: Option<CachedMetadataRecord>,
) -> Result<Arc<PackageMetadata>, DependencyError> {
    match registry.fetch_metadata_with_etag(package).await {
        Ok((metadata, etag)) => {
            if let Some(shared_cache) = shared_cache {
                let _ = shared_cache.write_metadata_record(
                    package,
                    &metadata,
                    etag,
                    current_unix_secs(),
                    None,
                    registry_url,
                );
            }
            Ok(Arc::new(metadata))
        }
        Err(error) => {
            if let Some(cached) = cached
                && metadata_record_is_usable_stale(&cached)
            {
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
                return Ok(Arc::new(cached.metadata));
            }

            Err(DependencyError(format!(
                "failed to fetch metadata for '{}': {}",
                package, error
            )))
        }
    }
}

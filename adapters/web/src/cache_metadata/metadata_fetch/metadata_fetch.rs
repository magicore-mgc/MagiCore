//! Public metadata-fetch facade and cache/network orchestration.
//! Facade tải metadata public và điều phối cache/network.

use std::sync::Arc;

use mgc_resolver::DependencyError;
use mgc_types::PackageName;

use crate::cache::SharedWebCache;
use crate::native;

use super::{
    fetch_with_stale_fallback, load_cached_metadata, refresh_cached_metadata, reuse_without_refresh,
};

pub async fn load_metadata_with_fallback(
    package: &PackageName,
    registry: &native::npm_registry::NpmRegistry,
    shared_cache: Option<&SharedWebCache>,
) -> Result<Arc<native::npm_registry::PackageMetadata>, DependencyError> {
    load_metadata_by_name_with_fallback(package.as_str(), registry, shared_cache).await
}

pub async fn load_metadata_by_name_with_fallback(
    package: &str,
    registry: &native::npm_registry::NpmRegistry,
    shared_cache: Option<&SharedWebCache>,
) -> Result<Arc<native::npm_registry::PackageMetadata>, DependencyError> {
    let registry_url = registry.registry_url().to_string();
    let cached = load_cached_metadata(package, registry, shared_cache, &registry_url)?;

    if let Some(cached_record) = cached.as_ref() {
        if let Some(metadata) = reuse_without_refresh(cached_record) {
            return Ok(metadata);
        }

        if let Some(result) = refresh_cached_metadata(
            package,
            registry,
            shared_cache,
            &registry_url,
            cached_record,
        )
        .await
        {
            return result;
        }
    }

    fetch_with_stale_fallback(package, registry, shared_cache, &registry_url, cached).await
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

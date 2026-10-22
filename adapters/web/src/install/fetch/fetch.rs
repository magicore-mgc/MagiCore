//! Cache-first coordinator for registry tarball retrieval.
//! Điều phối đọc cache trước rồi mới lấy tarball từ registry.

use std::sync::Arc;

use mgc_store::PackageCache;
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgError, MgResult};

use super::error::FetchError;
use super::{
    DownloadStagingGuard, package_tarball_url, publish_downloaded_tarball,
    unique_tarball_staging_path,
};
use crate::install::integrity::{
    prepare_verified_tarball_for_cache, verify_tarball_reader_integrity,
};
use crate::native;
use crate::profile::{TarballFetchResult, TarballPayload};

/// Fetch tarball bytes for a package (cache → shared cache → network).
/// Tải tarball bytes cho package (cache → shared cache → network).
///
/// Tries local cache first, then shared cache, then downloads from registry.
/// Thử cache local trước, rồi shared cache, rồi download từ registry.
pub async fn get_tarball_bytes(
    pkg: &ResolvedPackage,
    cache: &PackageCache,
    shared_package_cache: Option<&PackageCache>,
    registry_url: Option<&str>,
    registry_token: Option<&str>,
    download_sem: &tokio::sync::Semaphore,
) -> MgResult<TarballFetchResult> {
    let registry = registry_url.map(|url| {
        native::npm_registry::NpmRegistry::new_with_token(url, registry_token.map(str::to_string))
    });
    get_tarball_bytes_with_registry(
        pkg,
        cache,
        shared_package_cache,
        registry.as_ref(),
        download_sem,
    )
    .await
}

/// Keep credential policy attached to the instance on native artifact fetches.
/// Cache hit vẫn dùng được khi không có registry; network không làm mất scope.
pub async fn get_tarball_bytes_with_registry(
    pkg: &ResolvedPackage,
    cache: &PackageCache,
    shared_package_cache: Option<&PackageCache>,
    registry: Option<&native::npm_registry::NpmRegistry>,
    download_sem: &tokio::sync::Semaphore,
) -> MgResult<TarballFetchResult> {
    get_tarball_bytes_with_registry_inner(pkg, cache, shared_package_cache, registry, download_sem)
        .await
        .map_err(MgError::from)
}

async fn get_tarball_bytes_with_registry_inner(
    pkg: &ResolvedPackage,
    cache: &PackageCache,
    shared_package_cache: Option<&PackageCache>,
    registry: Option<&native::npm_registry::NpmRegistry>,
    download_sem: &tokio::sync::Semaphore,
) -> Result<TarballFetchResult, FetchError> {
    let prefer_shared_cache = shared_package_cache.is_some();

    // Try local cache first
    // Thử cache local trước
    if let Some(mut file) = cache
        .open_tarball(&pkg.id)
        .map_err(FetchError::LocalCacheRead)?
    {
        let len = file
            .metadata()
            .map_err(|error| FetchError::LocalCacheRead(error.into()))?
            .len();
        if verify_tarball_reader_integrity(pkg, &mut file).is_ok() {
            return Ok(TarballFetchResult {
                payload: TarballPayload::CachedPath(cache.tarball_path(&pkg.id), len),
                queue_wait_ms: 0,
                io_ms: 0,
                persist_to_shared_cache: false,
            });
        }
        drop(file);
        // Corrupted local cache: remove
        // Cache local hỏng: xóa
        let _ = std::fs::remove_file(cache.tarball_path(&pkg.id));
    }

    // Try shared cache
    // Thử shared cache
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(pc) = shared_package_cache
        && let Some(mut file) = pc
            .open_tarball(&pkg.id)
            .map_err(FetchError::SharedCacheRead)?
    {
        let len = file
            .metadata()
            .map_err(|error| FetchError::SharedCacheRead(error.into()))?
            .len();
        if verify_tarball_reader_integrity(pkg, &mut file).is_ok() {
            // Copy to local cache if needed
            // Copy vào local cache nếu cần
            if !prefer_shared_cache {
                let _ = cache.cache_tarball_from_path(&pkg.id, &pc.tarball_path(&pkg.id));
            }
            return Ok(TarballFetchResult {
                payload: TarballPayload::CachedPath(pc.tarball_path(&pkg.id), len),
                queue_wait_ms: 0,
                io_ms: 0,
                persist_to_shared_cache: false,
            });
        }
        drop(file);
        // Corrupted shared cache: remove
        // Shared cache hỏng: xóa
        let _ = std::fs::remove_file(pc.tarball_path(&pkg.id));
    }

    // Cache miss: download from registry
    // Cache miss: download từ registry
    let registry = registry.ok_or_else(|| FetchError::MissingRegistry {
        package: pkg.id.name_str().to_string(),
    })?;

    let queue_started_at = std::time::Instant::now();
    let _permit = download_sem
        .acquire()
        .await
        .map_err(FetchError::SemaphoreClosed)?;
    let queue_wait_ms = queue_started_at.elapsed().as_millis() as u64;

    let tarball_url = package_tarball_url(registry.registry_url(), pkg);
    let io_started_at = std::time::Instant::now();
    let mut pkg = pkg.clone();

    let final_path = shared_package_cache
        .map(|pc| pc.tarball_path(&pkg.id))
        .unwrap_or_else(|| cache.tarball_path(&pkg.id));
    let temp_path = unique_tarball_staging_path(&final_path);
    let staging = DownloadStagingGuard::new(temp_path.clone());

    // Download via NpmRegistry (supports zero-buffer streaming)
    // Download qua NpmRegistry (hỗ trợ zero-buffer streaming)
    let downloaded = registry
        .download_tarball_auto(&tarball_url, &temp_path)
        .await
        .map_err(|source| FetchError::RegistryDownload {
            package: pkg.id.name_str().to_string(),
            source,
        })?;

    let io_ms = io_started_at.elapsed().as_millis() as u64;

    match downloaded {
        // Small tarball: downloaded to memory
        // Tarball nhỏ: download vào memory
        native::npm_registry::DownloadedTarball::Bytes(bytes) => {
            prepare_verified_tarball_for_cache(&mut pkg, &bytes)?;

            let persist_to_shared_cache = shared_package_cache.is_some();
            if !persist_to_shared_cache {
                cache
                    .cache_tarball(&pkg.id, &bytes)
                    .map_err(FetchError::CacheWrite)?;
            }

            Ok(TarballFetchResult {
                payload: TarballPayload::Bytes(Arc::<[u8]>::from(bytes)),
                queue_wait_ms,
                io_ms,
                persist_to_shared_cache,
            })
        }
        // Large tarball: streamed to disk
        // Tarball lớn: stream vào disk
        native::npm_registry::DownloadedTarball::Streamed {
            computed_integrity,
            bytes_len,
        } => {
            // Verify streamed tarball integrity
            // Verify integrity tarball đã stream
            crate::install::integrity::verify_tarball_file_integrity(&pkg, staging.path())?;

            if pkg.integrity.is_empty() {
                pkg.integrity = computed_integrity.clone();
            }

            publish_downloaded_tarball(staging.path(), &final_path, &computed_integrity)?;

            Ok(TarballFetchResult {
                payload: TarballPayload::CachedPath(final_path, bytes_len),
                queue_wait_ms,
                io_ms,
                persist_to_shared_cache: false,
            })
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

//! Web package tarball prefetch behavior.
//! Hành vi prefetch tarball package Web.

use std::collections::HashSet;
use std::sync::Arc;

use mgc_store::PackageCache;
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgError, MgResult, PackageId};

use crate::cache::{SharedWebCache, download_concurrency_limit};
use crate::install::extract::tarball_prefetch_lock;
use crate::install::fetch::{
    DownloadStagingGuard, package_tarball_url, publish_downloaded_tarball,
    unique_tarball_staging_path,
};
use crate::install::integrity::{
    prepare_verified_tarball_for_cache, verify_tarball_reader_integrity,
};
use crate::native;

use super::PrefetchError;

enum PrefetchOutcome {
    CacheHit(u64),
    Downloaded(ResolvedPackage, Vec<u8>),
    StreamedToTemp {
        package: ResolvedPackage,
        staging: DownloadStagingGuard,
        computed_integrity: String,
    },
}

fn should_prefetch_package(package_id: &PackageId, skip: &HashSet<PackageId>) -> bool {
    !skip.contains(package_id)
}

pub async fn prefetch_tarballs(
    graph: &mgc_types::adapter::ResolvedGraph,
    skip: &std::collections::HashSet<PackageId>,
    cache: &PackageCache,
    shared_cache: Option<&SharedWebCache>,
    registry: &native::npm_registry::NpmRegistry,
) -> MgResult<u64> {
    let mut bytes_from_cache = 0u64;
    let shared_package_cache = shared_cache
        .map(|shared| shared.package_cache(registry.registry_url()))
        .transpose()
        .map_err(|error| MgError::Store(error.to_string()))?;
    let concurrency = download_concurrency_limit();
    let download_semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
    let mut pending_packages = graph
        .packages
        .iter()
        .filter(|package| should_prefetch_package(&package.id, skip))
        .cloned();
    let mut downloads = tokio::task::JoinSet::new();

    for package in pending_packages
        .by_ref()
        .take(initial_prefetch_window(graph.packages.len(), concurrency))
    {
        spawn_prefetch_task(
            &mut downloads,
            package,
            cache,
            shared_package_cache.as_ref(),
            &download_semaphore,
            registry,
        );
    }

    while let Some(joined) = downloads.join_next().await {
        let outcome = joined
            .map_err(PrefetchError::JoinTask)
            .map_err(MgError::from)??;
        match outcome {
            PrefetchOutcome::CacheHit(bytes) => {
                bytes_from_cache += bytes;
            }
            PrefetchOutcome::Downloaded(mut package, bytes) => {
                prepare_verified_tarball_for_cache(&mut package, &bytes)?;
                cache
                    .cache_tarball(&package.id, &bytes)
                    .map_err(|error| MgError::Store(error.to_string()))?;
                if let Some(shared_package_cache) = shared_package_cache.as_ref() {
                    let _ = shared_package_cache
                        .cache_tarball_from_path(&package.id, &cache.tarball_path(&package.id));
                }
            }
            PrefetchOutcome::StreamedToTemp {
                mut package,
                staging,
                computed_integrity,
            } => {
                crate::install::integrity::verify_tarball_file_integrity(&package, staging.path())?;
                if package.integrity.is_empty() {
                    package.integrity = computed_integrity.clone();
                }
                let final_path = shared_package_cache
                    .as_ref()
                    .map(|shared| shared.tarball_path(&package.id))
                    .unwrap_or_else(|| cache.tarball_path(&package.id));
                publish_downloaded_tarball(staging.path(), &final_path, &computed_integrity)?;
                if let Some(shared_package_cache) = shared_package_cache.as_ref() {
                    let _ = shared_package_cache.cache_tarball_from_path(&package.id, &final_path);
                }
            }
        }
        if let Some(package) = pending_packages.next() {
            spawn_prefetch_task(
                &mut downloads,
                package,
                cache,
                shared_package_cache.as_ref(),
                &download_semaphore,
                registry,
            );
        }
    }

    Ok(bytes_from_cache)
}

fn initial_prefetch_window(package_count: usize, concurrency: usize) -> usize {
    package_count.min(concurrency.max(1))
}

fn spawn_prefetch_task(
    downloads: &mut tokio::task::JoinSet<MgResult<PrefetchOutcome>>,
    package: ResolvedPackage,
    cache: &PackageCache,
    shared_package_cache: Option<&PackageCache>,
    download_semaphore: &Arc<tokio::sync::Semaphore>,
    registry: &native::npm_registry::NpmRegistry,
) {
    let local_cache = cache.clone();
    let shared_package_cache = shared_package_cache.cloned();
    let download_semaphore = Arc::clone(download_semaphore);
    let registry = registry.clone_for_downloads();
    downloads.spawn(async move {
        let lock = tarball_prefetch_lock(&package.id);
        let _guard = lock.lock().await;
        let _permit = download_semaphore
            .acquire_owned()
            .await
            .map_err(PrefetchError::SemaphoreClosed)
            .map_err(MgError::from)?;

        if let Some(file) = local_cache
            .open_tarball(&package.id)
            .map_err(|error| MgError::Store(error.to_string()))?
        {
            let len = file
                .metadata()
                .map_err(|error| MgError::Store(error.to_string()))?
                .len();
            if verify_tarball_reader_integrity(&package, file).is_ok() {
                return Ok(PrefetchOutcome::CacheHit(len));
            }
            let _ = std::fs::remove_file(local_cache.tarball_path(&package.id));
        }

        // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
        if let Some(shared_package_cache) = shared_package_cache.as_ref()
            && let Some(file) = shared_package_cache
                .open_tarball(&package.id)
                .map_err(|error| MgError::Store(error.to_string()))?
        {
            let len = file
                .metadata()
                .map_err(|error| MgError::Store(error.to_string()))?
                .len();
            if verify_tarball_reader_integrity(&package, file).is_ok() {
                local_cache
                    .cache_tarball_from_path(
                        &package.id,
                        &shared_package_cache.tarball_path(&package.id),
                    )
                    .map_err(|error| MgError::Store(error.to_string()))?;
                return Ok(PrefetchOutcome::CacheHit(len));
            }
            let _ = std::fs::remove_file(shared_package_cache.tarball_path(&package.id));
        }

        let url = package_tarball_url(registry.registry_url(), &package);
        // One authenticated GET selects the byte or streaming path and provides its real size.
        // Một GET đã xác thực chọn nhánh byte hoặc stream và cung cấp kích thước thật.
        let staging_path = unique_tarball_staging_path(&local_cache.tarball_path(&package.id));
        let staging = DownloadStagingGuard::new(staging_path);
        let downloaded = registry
            .download_tarball_auto(&url, staging.path())
            .await
            .map_err(|error| {
                MgError::Network(format!(
                    "download failed for '{}': {}",
                    package.id.name_str(),
                    error
                ))
            })?;
        match downloaded {
            native::npm_registry::DownloadedTarball::Bytes(bytes) => {
                Ok(PrefetchOutcome::Downloaded(package, bytes))
            }
            native::npm_registry::DownloadedTarball::Streamed {
                computed_integrity, ..
            } => Ok(PrefetchOutcome::StreamedToTemp {
                package,
                staging,
                computed_integrity,
            }),
        }
    });
}

#[cfg(test)]
#[path = "test/prefetch.rs"]
mod tests;

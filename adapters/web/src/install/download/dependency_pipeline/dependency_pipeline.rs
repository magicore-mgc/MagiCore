//! Web resolved dependency download and extraction behavior.
//! Hành vi tải và giải nén dependency Web đã resolve.

use std::io::{Seek, SeekFrom};
use std::sync::Arc;

use futures_util::stream::{self, StreamExt};
use mgc_store::{ContentStore, Layout, PackageCache};
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgError, MgResult, PackageId};

use crate::cache::{SharedWebCache, download_concurrency_limit, extract_concurrency_limit};
use crate::install::extract::{ensure_extracted_package_root_from_bytes, tarball_prefetch_lock};
use crate::install::fetch::get_tarball_bytes_with_registry;
use crate::install::integrity::verify_tarball_reader_integrity;
use crate::native;
use crate::profile::{PipelineProfile, TarballPayload};

use super::super::task_concurrency::pipeline_task_concurrency_limit;
use super::PipelineError;
use super::result_aggregation::aggregate_pipeline_results;

fn should_schedule_pipeline_package(
    package_id: &PackageId,
    skip: &std::collections::HashSet<PackageId>,
) -> bool {
    !skip.contains(package_id)
}

#[allow(clippy::too_many_arguments)]
pub async fn pipeline_download_and_extract(
    graph: &mgc_types::adapter::ResolvedGraph,
    skip: &std::collections::HashSet<PackageId>,
    cache: &PackageCache,
    shared_cache: Option<&SharedWebCache>,
    registry: Option<&native::npm_registry::NpmRegistry>,
    layout: &Layout,
    store: &ContentStore,
    generation: i64,
) -> MgResult<(
    u64,
    std::collections::HashMap<PackageId, std::path::PathBuf>,
    Vec<tokio::task::JoinHandle<()>>,
)> {
    let download_sem = Arc::new(tokio::sync::Semaphore::new(download_concurrency_limit()));
    let pipeline_profile = Arc::new(PipelineProfile::from_env());
    let shared_package_cache = shared_cache
        .map(|shared| shared.package_cache())
        .transpose()
        .map_err(|e| MgError::Store(e.to_string()))?;
    let extract_concurrency = extract_concurrency_limit();
    let extract_sem = Arc::new(tokio::sync::Semaphore::new(extract_concurrency));
    let task_concurrency = pipeline_task_concurrency_limit(extract_concurrency);
    let scheduled_packages: Vec<ResolvedPackage> = graph
        .packages
        .iter()
        .filter(|pkg| should_schedule_pipeline_package(&pkg.id, skip))
        .cloned()
        .collect();
    let tasks = scheduled_packages.into_iter().map(|pkg| {
        let cache = cache.clone();
        let shared_cache = shared_cache.cloned();
        let shared_package_cache = shared_package_cache.clone();
        let registry = registry.map(|registry| registry.clone_for_downloads());
        let layout = layout.clone();
        let store = store.clone();
        let download_sem = Arc::clone(&download_sem);
        let extract_sem = Arc::clone(&extract_sem);
        let pipeline_profile = Arc::clone(&pipeline_profile);

        async move {
            let lock = tarball_prefetch_lock(&pkg.id);
            let _guard = lock.lock().await;

            let download_started_at = std::time::Instant::now();
            let fetch = get_tarball_bytes_with_registry(
                &pkg,
                &cache,
                shared_package_cache.as_ref(),
                registry.as_ref(),
                &download_sem,
            )
            .await?;
            // Test-only failpoint (Gate 11-B.2): park after the first
            // package's tarball is obtained (cache or network) but before
            // extract — the staging generation still has zero claims.
            // (Failpoint chỉ-cho-test: đỗ sau khi lấy được tarball của gói
            // đầu tiên (cache hoặc network) nhưng trước khi extract — staging
            // generation vẫn chưa có claim.)
            mgc_store::failpoint::hit("after-fetch");
            pipeline_profile.record_download(
                &pkg.id,
                fetch.payload.len(),
                download_started_at.elapsed().as_millis() as u64,
                fetch.queue_wait_ms,
                fetch.io_ms,
            );

            let shared_cache_persist = if fetch.persist_to_shared_cache {
                shared_package_cache.clone().map(|pc| {
                    let pkg_id = pkg.id.clone();
                    match &fetch.payload {
                        TarballPayload::Bytes(bytes) => {
                            let bytes = Arc::clone(bytes);
                            tokio::task::spawn_blocking(move || {
                                let _ = pc.cache_tarball(&pkg_id, bytes.as_ref());
                            })
                        }
                        TarballPayload::CachedPath(path, _) => {
                            let path = path.clone();
                            tokio::task::spawn_blocking(move || {
                                let _ = pc.cache_tarball_from_path(&pkg_id, &path);
                            })
                        }
                    }
                })
            } else {
                None
            };

            let _permit = extract_sem
                .acquire_owned()
                .await
                .map_err(PipelineError::ExtractSemaphoreClosed)
                .map_err(MgError::from)?;
            let id = pkg.id.clone();
            let tarball_len = fetch.payload.len();
            let extract_started_at = std::time::Instant::now();
            let root = tokio::task::spawn_blocking(move || match fetch.payload {
                TarballPayload::Bytes(bytes) => ensure_extracted_package_root_from_bytes(
                    &layout,
                    &store,
                    shared_cache.as_ref(),
                    &pkg,
                    bytes.as_ref(),
                    generation,
                ),
                TarballPayload::CachedPath(path, _) => {
                    let mut file = mgc_platform::open_regular_file_no_follow(&path)
                        .map_err(|error| MgError::Store(error.to_string()))?;
                    if let Err(error) = verify_tarball_reader_integrity(&pkg, &mut file) {
                        drop(file);
                        let _ = std::fs::remove_file(&path);
                        return Err(error);
                    }
                    file.seek(SeekFrom::Start(0))
                        .map_err(|error| MgError::Store(error.to_string()))?;
                    crate::install::extract::ensure_extracted_package_root_from_file(
                        &layout,
                        &store,
                        shared_cache.as_ref(),
                        &pkg,
                        file,
                        generation,
                    )
                }
            })
            .await
            .map_err(PipelineError::ExtractionTaskPanicked)
            .map_err(MgError::from)??;
            pipeline_profile.record_extract(&id, extract_started_at.elapsed().as_millis() as u64);

            Ok::<_, MgError>((tarball_len, id, root, shared_cache_persist))
        }
    });

    let finished = stream::iter(tasks)
        .buffer_unordered(task_concurrency)
        .collect::<Vec<_>>()
        .await;
    let aggregated = aggregate_pipeline_results(finished)?;
    pipeline_profile.flush();
    Ok(aggregated)
}

#[cfg(test)]
#[path = "test/dependency_pipeline.rs"]
mod tests;

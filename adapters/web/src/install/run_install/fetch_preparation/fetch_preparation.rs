//! Lock intent, local cache pruning, and legacy-flat prefetch.
//! Ghi lock intent, dọn cache local và prefetch legacy-flat.

use crate::cache::prune_project_local_cache;
use crate::install::download::prefetch_tarballs;
use crate::install::run_install::state::InstallState;
use crate::install::snapshot_prior_lock;
use crate::lockfile::write_web_lockfile_with_state;
use mgc_types::{MgError, MgResult};

pub(in crate::install::run_install) async fn prepare(state: &mut InstallState<'_>) -> MgResult<()> {
    state.prior_lock = snapshot_prior_lock(state.project_root)?;
    write_web_lockfile_with_state(state.project_root, state.graph, state.registry_url)
        .inspect_err(|_| {
            if let Some(staging_root) = &state.staging_root {
                let _ = std::fs::remove_dir_all(staging_root);
            }
        })?;
    eprintln!("[magicore:debug] install:prune_cache_start");
    prune_project_local_cache(&state.layout);
    state.profile.mark("prune_project_local_cache", state.start);

    if state.opts.legacy_flat {
        if let Some(handle) = state.prefetch_handle.take() {
            match handle.await {
                Ok(Ok(bytes)) => state.summary.bytes_from_cache += bytes,
                Ok(Err(error)) => {
                    if let Some(staging_root) = &state.staging_root {
                        let _ = std::fs::remove_dir_all(staging_root);
                    }
                    return Err(error);
                }
                Err(error) => {
                    if let Some(staging_root) = &state.staging_root {
                        let _ = std::fs::remove_dir_all(staging_root);
                    }
                    return Err(MgError::Other(format!("prefetch panicked: {error}")));
                }
            }
        }
    } else if let Some(handle) = state.prefetch_handle.take() {
        handle.abort();
    }

    if state.opts.legacy_flat && !state.fetch_graph.is_empty() {
        let active_package_cache = state.active_package_cache();
        let secondary_shared_cache = state.secondary_shared_cache();
        let bytes = prefetch_tarballs(
            &state.fetch_graph,
            &state.already_materialized,
            active_package_cache,
            secondary_shared_cache,
            state.registry,
        )
        .await
        .inspect_err(|_| {
            if let Some(staging_root) = &state.staging_root {
                let _ = std::fs::remove_dir_all(staging_root);
            }
        })?;
        state.summary.bytes_from_cache += bytes;
    }
    state.profile.mark("prefetch_tarballs", state.start);
    Ok(())
}

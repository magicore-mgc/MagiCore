//! Stable Web install entry points and ordered phase coordination.
//! Entry point Web install ổn định và điều phối phase theo thứ tự.

use crate::cache::SharedWebCache;
use crate::native::npm_registry::NpmRegistry;
use mgc_store::ContentStore;
use mgc_types::MgResult;
use mgc_types::adapter::{InstallOptions, InstallSummary, ResolvedGraph};
use std::path::Path;

#[allow(clippy::too_many_arguments)]
pub async fn run_install(
    registry_url: &str,
    provider_auth_token: Option<&str>,
    store_override: Option<&ContentStore>,
    shared_cache: Option<SharedWebCache>,
    prefetch_handle: Option<tokio::task::JoinHandle<MgResult<u64>>>,
    graph: &ResolvedGraph,
    project_root: &Path,
    opts: InstallOptions,
) -> MgResult<InstallSummary> {
    let registry =
        NpmRegistry::new_with_token(registry_url, provider_auth_token.map(str::to_string));
    run_install_with_registry(
        &registry,
        store_override,
        shared_cache,
        prefetch_handle,
        graph,
        project_root,
        opts,
    )
    .await
}

/// Preserve the configured transport and credential scope across installation.
/// Giữ nguyên transport và phạm vi credential từ provider xuyên qua installer.
#[allow(clippy::too_many_arguments)]
pub async fn run_install_with_registry(
    registry: &NpmRegistry,
    store_override: Option<&ContentStore>,
    shared_cache: Option<SharedWebCache>,
    prefetch_handle: Option<tokio::task::JoinHandle<MgResult<u64>>>,
    graph: &ResolvedGraph,
    project_root: &Path,
    opts: InstallOptions,
) -> MgResult<InstallSummary> {
    let mut state = super::setup::initialize(
        registry,
        store_override,
        shared_cache,
        prefetch_handle,
        graph,
        project_root,
        opts,
    )?;

    if super::fast_path::try_noop(&mut state)? {
        return Ok(state.summary);
    }
    super::package_plan::prepare(&mut state)?;
    super::fetch_preparation::prepare(&mut state).await?;
    super::materialization::materialize(&mut state).await?;
    super::lifecycle::run(&mut state).await?;
    super::commit::finish(&mut state)?;
    Ok(state.summary)
}

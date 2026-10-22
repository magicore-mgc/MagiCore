//! Runs one workspace package through the mutation gateway before resolve/install.
//! Chạy một package workspace qua mutation gateway trước resolve/install.

use anyhow::Result;
use mgc_types::adapter::PackageAdapter;
use std::path::Path;

use crate::commands::core::shared;

pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) async fn install_web_target_quiet(
    adapter: &dyn PackageAdapter,
    target: &Path,
    frozen: bool,
    opts: mgc_types::adapter::InstallOptions,
) -> Result<()> {
    // Each workspace target is its own project: acquire its mutation gateway
    // before resolution and the empty-graph return, so recovery and concurrent
    // installs cannot bypass that target's writer lock.
    // (Mỗi target là một project riêng: vào mutation gateway trước resolve
    // và nhánh graph rỗng để recovery/install đồng thời không bypass writer lock.)
    let _write_lock =
        shared::begin_dependency_mutation(adapter, target, shared::MutationOperation::Install)
            .await?;
    let execution =
        shared::prepare_install_execution(adapter, target, frozen || opts.offline, None).await?;
    if execution.graph.is_empty() {
        return Ok(());
    }
    adapter.install(&execution.graph, target, opts).await?;
    Ok(())
}

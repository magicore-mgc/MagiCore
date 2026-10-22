//! Executes topological Web workspace package installs with bounded parallelism.
//! Thực thi cài package workspace Web theo topo với song song có giới hạn.

use anyhow::Result;
use mgc_types::adapter::PackageAdapter;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::super::super::super::super::compat_install_target;
use super::super::concurrency_policy::monorepo_install_concurrency;
use super::super::workspace_target_install::install_web_target_quiet;
use crate::commands::core::shared::should_use_legacy_flat_layout;

pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install)
struct MonorepoInstallParams
{
    pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) adapter:
        Arc<dyn PackageAdapter>,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) targets:
        Vec<PathBuf>,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) frozen:
        bool,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) offline:
        bool,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) ignore_scripts:
        bool,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) allow_scripts:
        bool,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) prefer_dedupe:
        bool,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) repair:
        bool,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) compat:
        crate::commands::compat::CompatMode,
}

pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) async fn install_monorepo_targets(
    params: MonorepoInstallParams,
) -> Result<()> {
    let mut native_targets = Vec::new();
    let mut package_targets = Vec::new();

    for target in &params.targets {
        if target.join("package.json").exists() {
            package_targets.push(target.clone());
        } else {
            native_targets.push(target.clone());
        }
    }

    if params.offline && !native_targets.is_empty() {
        anyhow::bail!(
            "offline install cannot use delegated non-package workspace members; no workspace install was started"
        );
    }

    if !package_targets.is_empty() {
        let graph = mgc_workspace::build_workspace_graph(&package_targets)?;
        let levels = mgc_workspace::dependency_first_levels(&graph)
            .map_err(|e| crate::error::topo_order_failed(&e))?;

        let concurrency = monorepo_install_concurrency(&package_targets);
        let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));

        for level in &levels {
            let mut join_set = tokio::task::JoinSet::new();
            for &node_index in level {
                let node = graph.nodes[node_index].clone();
                let adapter = Arc::clone(&params.adapter);
                let semaphore = Arc::clone(&semaphore);
                join_set.spawn(async move {
                    let _permit = semaphore
                        .acquire_owned()
                        .await
                        .map_err(|e| crate::error::install_slot_failed(&e))?;
                    mgc_ui::info(&format!("Installing workspace: {}", node.path.display()));
                    let install_options = mgc_types::adapter::InstallOptions {
                        ignore_scripts: params.ignore_scripts,
                        allow_scripts: params.allow_scripts,
                        prefer_dedupe: params.prefer_dedupe,
                        repair: params.repair,
                        legacy_flat: should_use_legacy_flat_layout("web"),
                        frozen: params.frozen,
                        offline: params.offline,
                        ..Default::default()
                    };
                    install_web_target_quiet(
                        adapter.as_ref(),
                        &node.path,
                        params.frozen,
                        install_options,
                    )
                    .await?;
                    Ok::<PathBuf, anyhow::Error>(node.path)
                });
            }
            while let Some(result) = join_set.join_next().await {
                result.map_err(|e| crate::error::install_task_failed(&e))??;
            }
        }
    }

    for target in native_targets {
        compat_install_target(&target, &params.compat)?;
    }

    Ok(())
}

// Issue #4: Disabled due to lockfile v2 migration (uses LockPackage, WorkspaceLock, ResolutionMeta)
pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) fn write_monorepo_root_lockfile(
    _project_root: &Path,
    _targets: &[PathBuf],
) -> Result<()> {
    // Workspace lockfile merging requires v2 schema rewrite
    // For now, each workspace maintains its own lockfile
    Ok(())
}

//! Executes a mixed Web/multi-core workspace install.
//! Thực thi install workspace Web/multi-core.

use anyhow::Result;
use std::sync::Arc;

use super::super::options::WebInstallOptions;
use super::super::plan::WebInstallPlan;
use super::link_workspace_packages::link_monorepo_workspace_packages;
use super::target_installer::{
    MonorepoInstallParams, install_monorepo_targets, write_monorepo_root_lockfile,
};

/// Runs only the monorepo execution branch after the shared install plan is ready.
/// Chạy riêng nhánh monorepo sau khi install plan chung đã sẵn sàng.
pub(in crate::commands::core::web::implementation::package_lifecycle::install) async fn run(
    plan: &WebInstallPlan,
    options: &WebInstallOptions,
) -> Result<()> {
    let mut web_targets = Vec::new();
    let mut mix_targets = Vec::new();
    for target in &plan.targets {
        if target.join("package.json").exists() {
            web_targets.push(target.clone());
        } else {
            mix_targets.push(target.clone());
        }
    }

    if options.offline && !mix_targets.is_empty() {
        anyhow::bail!(
            "offline install for mixed-core monorepos is not yet supported; no workspace install was started"
        );
    }

    if !web_targets.is_empty() {
        install_monorepo_targets(MonorepoInstallParams {
            adapter: Arc::clone(&plan.adapter),
            targets: web_targets.clone(),
            frozen: options.frozen,
            offline: options.offline,
            ignore_scripts: options.ignore_scripts,
            allow_scripts: options.allow_scripts,
            prefer_dedupe: options.prefer_dedupe,
            repair: options.repair,
            compat: plan.compat.clone(),
        })
        .await?;
        link_monorepo_workspace_packages(&plan.root, &web_targets)?;
        write_monorepo_root_lockfile(&plan.root, &web_targets)?;
    }

    for target in &mix_targets {
        let ctx = crate::context::ProjectContext::load_for_dir(target)?;
        mgc_ui::info(&format!("Installing workspace: {}", target.display()));
        crate::commands::install::install_into_root_ws(
            ctx.adapter(),
            target,
            &options.packages,
            options.ignore_scripts,
            options.allow_scripts,
            options.offline,
            options.frozen,
        )
        .await?;
    }
    Ok(())
}

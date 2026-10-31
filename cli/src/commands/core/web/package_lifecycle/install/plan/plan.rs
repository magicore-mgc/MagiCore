//! Resolves compatibility, adapter, targets, dedupe policy, and root package mutation.
//! Resolve compat, adapter, target, dedupe và mutation package tại root.

use anyhow::Result;
use mgc_types::adapter::PackageAdapter;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::super::super::super::{compat_install_target, install_targets};
use super::super::options::WebInstallOptions;
use crate::commands::compat::CompatMode;
use crate::commands::core::shared;
use crate::commands::core::web::implementation::project_context::adapter_selection::web_adapter;
use crate::commands::core::web::implementation::project_context::is_startable_server_half;

/// State computed once and consumed by exactly one execution lane.
/// Trạng thái được tính một lần rồi dùng bởi đúng một lane thực thi.
pub(in crate::commands::core::web::implementation::package_lifecycle::install) struct WebInstallPlan
{
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) root: PathBuf,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) adapter:
        Arc<dyn PackageAdapter>,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) targets:
        Vec<PathBuf>,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) compat:
        CompatMode,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) dedupe_enabled:
        bool,
}

pub(in crate::commands::core::web::implementation::package_lifecycle::install) async fn prepare(
    root: &Path,
    options: &WebInstallOptions,
) -> Result<WebInstallPlan> {
    let compat = crate::commands::dep_gate::from_dep_flag(options.compat_runtime.as_deref())?;
    crate::commands::dep_gate::gate(
        &crate::commands::dep_gate::DepContext::new(
            "web",
            Some(crate::commands::dep_gate::eco::JS),
            None,
            None,
            crate::commands::dep_gate::DepOp::Install,
        ),
        None,
        &compat,
        Some(&root.join(".magicore").join("exec.log")),
    )?;
    let adapter: Arc<dyn PackageAdapter> = web_adapter(root)?;
    let targets = install_targets(root)?;

    // Reject unsupported mixed-stack members before any frontend manifest,
    // lockfile, or shared-store mutation.
    // A startable server half is started by its own runner, so this installer
    // has nothing to resolve for it and skips it explicitly. Anything else
    // without a JS manifest still fails closed.
    // (Nửa server khởi động được do runner riêng chạy nên installer này không có
    // gì để resolve và bỏ qua tường minh. Thư mục khác thiếu manifest JS vẫn
    // fail-closed.)
    for target in &targets {
        if target.join("package.json").exists() || is_startable_server_half(target) {
            continue;
        }
        compat_install_target(target, &compat)?;
    }

    // Dedupe is opt-in from CLI or mgc.toml [dedupe].
    let mut dedupe_enabled = options.prefer_dedupe;
    if !dedupe_enabled && let Ok(Some(cfg)) = mgc_config::project::ProjectConfig::load(root) {
        dedupe_enabled = cfg.dedupe.prefer;
    }
    if dedupe_enabled {
        adapter.set_dedupe_pref(true);
        let mut lock_roots: Vec<&Path> = vec![root];
        lock_roots.extend(
            targets
                .iter()
                .filter(|target| target.join("package.json").exists())
                .map(|target| target.as_path()),
        );
        // Existing-version lock seeding remains disabled until lockfile v2 migration.
        if let Ok(_existing) = Ok::<Vec<String>, ()>(Vec::new()) {
            // Keep the previous explicit no-op until the migration is complete.
        }
    }

    // Root changes use the shared lock/journal mutation gateway.
    if !options.packages.is_empty() {
        shared::add(
            adapter.as_ref(),
            root,
            options.packages.clone(),
            None,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
        )
        .await?;
    }

    Ok(WebInstallPlan {
        root: root.to_path_buf(),
        adapter,
        targets,
        compat,
        dedupe_enabled,
    })
}

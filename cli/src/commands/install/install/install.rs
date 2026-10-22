use crate::context::ProjectContext;
use anyhow::Result;
use mgc_ui::{info, success};

use super::super::mutation::install_into_root;
use super::super::workspace::discover_workspace_projects;

/// mgc install — install dependencies for the current project
pub async fn run(
    packages: Vec<String>,
    core: Option<&str>,
    ignore_scripts: bool,
    allow_scripts: bool,
    offline: bool, // T4.1: offline mode flag
    frozen: bool,  // Frozen mode: fail if lockfile needs update (CI mode)
) -> Result<()> {
    // T4.1: Offline mode — thread env flag; adapters read MGC_OFFLINE_MODE
    // (web resolve gate) and receive InstallOptions.offline (install gate).
    if offline {
        crate::offline::set_offline_mode(true);
    }

    let ctx = ProjectContext::load_with_core(core)?;
    let adapter = ctx.adapter();

    // Pre-install hooks (mgc.hooks.toml [hooks.pre-install]) — fail → abort install
    // (21 §9 fail-closed; hooks cannot bypass security checks).
    crate::commands::hooks::run_event(ctx.root(), "pre-install")?;

    if let Some(workspaces) = discover_workspace_projects(ctx.root())? {
        if workspaces.is_empty() {
            info("No installable workspaces found in this monorepo.");
            return Ok(());
        }

        // R3 FIX (AUDIT VÒNG 2): Pre-validate ALL workspaces in offline mode
        if offline {
            let mut missing_lockfiles = Vec::new();
            for ws in &workspaces {
                let lockfile = ws.join("mgc.lock");
                if !lockfile.exists() {
                    missing_lockfiles.push(ws.display().to_string());
                }
            }

            if !missing_lockfiles.is_empty() {
                anyhow::bail!(
                    "Offline mode requires lockfiles in all {} workspaces.\n  \
                     Missing lockfiles:\n  {}",
                    workspaces.len(),
                    missing_lockfiles.join("\n  ")
                );
            }

            info(&format!(
                "✓ All {} workspaces have lockfiles (offline mode)",
                workspaces.len()
            ));
        }

        if !packages.is_empty() {
            info("Applying requested packages to each detected workspace.");
        }

        // ITEM 7: install workspace song song (buffered 4), lỗi 1 project không chặn repo
        use futures_util::StreamExt;
        let results: Vec<Result<()>> = futures_util::stream::iter(workspaces)
            .map(|workspace| {
                let packages = &packages;
                async move {
                    info(&format!("Installing workspace: {}", workspace.display()));
                    // Mix core (Q23): detect core riêng cho từng project — không dùng
                    // adapter của root (root có thể không thuộc core nào).
                    let ctx = crate::context::ProjectContext::load_for_dir(&workspace)?;
                    let adapter = ctx.adapter();
                    install_into_root(
                        adapter,
                        &workspace,
                        packages,
                        ignore_scripts,
                        allow_scripts,
                        offline,
                        frozen, // frozen mode
                    )
                    .await
                }
            })
            .buffered(4)
            .collect()
            .await;

        let mut failed = 0usize;
        for result in results {
            if let Err(e) = result {
                failed += 1;
                mgc_ui::error(&format!("Workspace install failed: {e:#}"));
            }
        }

        mgc_ui::blank_line();
        if failed > 0 {
            return Err(crate::error::workspace_failed(failed));
        }
        success("Workspace dependencies installed");
        return Ok(());
    }

    install_into_root(
        adapter,
        ctx.root(),
        &packages,
        ignore_scripts,
        allow_scripts,
        offline, // T4.1
        frozen,  // frozen mode
    )
    .await
}

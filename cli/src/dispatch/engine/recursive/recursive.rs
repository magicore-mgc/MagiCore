// Recursive workspace coordinator — select, order, validate, then dispatch targets.
// Điều phối workspace recursive — lọc, sắp thứ tự, xác minh rồi dispatch target.
use super::super::command_name::command_name;
use super::super::dispatch_command::dispatch_command;
use super::execution_plan::{WorkspaceExecutionTarget, plan_workspaces};
use super::workspace_filter::filter_workspaces;
use super::workspace_order::order_workspaces;
use crate::Commands;
use anyhow::{Result, bail};

/// Run an already-supported command across the selected monorepo workspaces.
/// Chạy command đã hỗ trợ trên các workspace được chọn trong monorepo.
pub(in crate::dispatch) async fn run_recursive(
    command: Commands,
    core: Option<&str>,
    filter: Option<&str>,
) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = mgc_config::project::ProjectConfig::find_project_root(&cwd)
        .ok_or_else(crate::error::project_root_missing)?;
    let workspaces = crate::commands::install::discover_workspace_projects(&project_root)?
        .ok_or_else(|| {
            anyhow::anyhow!("--recursive requires magicore.workspace.toml (mode = \"monorepo\")")
        })?;

    let workspaces = if let Some(pattern) = filter {
        let selected = filter_workspaces(&project_root, &workspaces, pattern)?;
        if selected.is_empty() {
            mgc_ui::info(&format!(
                "--filter '{pattern}' matched no workspace packages."
            ));
            return Ok(());
        }
        mgc_ui::info(&format!(
            "--filter '{pattern}' → {} workspace(s)",
            selected.len()
        ));
        selected
    } else {
        workspaces
    };

    if workspaces.is_empty() {
        bail!("No workspace projects found in this monorepo.");
    }

    let workspaces = order_workspaces(workspaces)?;
    // Validate every selected project before invoking the first mutating handler.
    // Xác minh mọi project trước khi gọi handler có thể thay đổi dữ liệu.
    let targets = plan_workspaces(workspaces, &command, core)?;

    let name = command_name(&command);
    let mut failed = 0usize;
    for target in targets {
        let WorkspaceExecutionTarget {
            path,
            core: ws_core,
        } = target;
        let ws_core_str = ws_core.as_deref();

        mgc_ui::info(&format!(
            "== {name} → {} (core: {})",
            path.display(),
            ws_core_str.unwrap_or("auto")
        ));
        let result = async {
            std::env::set_current_dir(&path)?;
            dispatch_command(command.clone(), ws_core_str, false).await
        }
        .await;
        if let Err(error) = result {
            failed += 1;
            mgc_ui::error(&format!("{name} failed in '{}': {error:#}", path.display()));
        }
    }

    std::env::set_current_dir(&cwd)?;
    if failed > 0 {
        return Err(crate::error::workspace_failed(failed));
    }
    Ok(())
}

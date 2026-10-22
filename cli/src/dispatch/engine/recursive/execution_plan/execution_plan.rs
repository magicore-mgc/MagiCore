//! Validate explicit ownership and resolve the core for every selected workspace.
//! Xác minh owner tường minh và resolve core cho mọi workspace được chọn.

use super::super::super::identity::{
    explicit_dependency_core, resolve_recursive_workspace_core, validate_explicit_dependency_core,
};
use crate::Commands;
use anyhow::Result;
use std::path::PathBuf;

/// A workspace path paired with its already-validated dispatch core.
/// Path workspace đi cùng core đã được xác minh để dispatch.
pub(in crate::dispatch::engine::recursive) struct WorkspaceExecutionTarget {
    pub(in crate::dispatch::engine::recursive) path: PathBuf,
    pub(in crate::dispatch::engine::recursive) core: Option<String>,
}

/// Build all dispatch targets before any workspace command starts.
/// Tạo toàn bộ dispatch target trước khi chạy command workspace đầu tiên.
pub(in crate::dispatch::engine::recursive) fn plan_workspaces(
    workspaces: Vec<PathBuf>,
    command: &Commands,
    global_core: Option<&str>,
) -> Result<Vec<WorkspaceExecutionTarget>> {
    let command_core = explicit_dependency_core(command);
    let mut targets = Vec::with_capacity(workspaces.len());
    for path in workspaces {
        if let Some(command_core) = command_core {
            validate_explicit_dependency_core(&path, command_core, global_core)?;
        }
        let core = resolve_recursive_workspace_core(&path, global_core)?;
        targets.push(WorkspaceExecutionTarget { path, core });
    }
    Ok(targets)
}

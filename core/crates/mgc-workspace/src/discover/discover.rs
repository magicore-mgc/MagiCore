//! Workspace target discovery coordinator.
//! Điều phối việc tìm các project thuộc workspace.

use super::{
    layout_path::resolve_workspace_layout_dir, project_scan::collect_projects,
    workspace_config::workspace_layout,
};
use std::path::{Path, PathBuf};

/// Discover package projects under the configured application and package roots.
/// Tìm package project dưới các thư mục ứng dụng và package đã cấu hình.
pub fn discover_workspace_targets(project_root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let options = workspace_layout(project_root)?;
    let mut targets = Vec::new();
    let apps_dir = resolve_workspace_layout_dir(project_root, &options.apps_dir, "apps_dir")?;
    let packages_dir =
        resolve_workspace_layout_dir(project_root, &options.packages_dir, "packages_dir")?;
    collect_projects(apps_dir, &mut targets)?;
    collect_projects(packages_dir, &mut targets)?;
    targets.sort();
    targets.dedup();
    Ok(targets)
}

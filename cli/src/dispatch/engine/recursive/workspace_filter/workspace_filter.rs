//! Filter workspace targets by package name or relative path.
//! Lọc workspace target theo tên package hoặc đường dẫn tương đối.

use super::super::super::identity::workspace_package_name;
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Return matching paths, falling back to directory names only when manifests are absent.
/// Trả path khớp; chỉ dùng tên thư mục dự phòng khi manifest thật sự vắng.
pub(in crate::dispatch::engine::recursive) fn filter_workspaces(
    project_root: &Path,
    workspaces: &[PathBuf],
    pattern: &str,
) -> Result<Vec<PathBuf>> {
    let mut selected = Vec::new();
    for workspace in workspaces {
        let relative = workspace.strip_prefix(project_root).unwrap_or(workspace);
        let name = workspace_package_name(workspace)?.unwrap_or_else(|| {
            workspace
                .file_name()
                .map(|part| part.to_string_lossy().into_owned())
                .unwrap_or_default()
        });
        if mgc_workspace::filter_matches(pattern, relative, &name) {
            selected.push(workspace.clone());
        }
    }
    Ok(selected)
}

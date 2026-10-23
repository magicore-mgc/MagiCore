//! Project discovery and temporary mirror for Web builds.
//! Tìm dự án và tạo mirror tạm cho build Web.
use anyhow::Result;

#[path = "project_mirror/mod.rs"]
mod project_mirror;

use project_mirror::{link_node_modules_for_build, mirror_project_for_build};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Keeps the temporary directory alive for the duration of the build operation.
/// Giữ thư mục tạm tồn tại trong suốt thao tác build.
pub(in crate::bundler) struct PreparedWorkspace {
    pub(in crate::bundler) working_dir: PathBuf,
    pub(in crate::bundler) entry: PathBuf,
    _temp_root: Option<TempDir>,
}

/// Finds the nearest supported project manifest above an entry file.
/// Tìm manifest dự án được hỗ trợ gần nhất phía trên file entry.
pub(in crate::bundler) fn detect_working_dir(entry: &Path) -> Option<PathBuf> {
    let mut current = entry.parent()?;
    loop {
        if current.join("package.json").exists()
            || current.join("mgc.toml").exists()
            || current.join("Cargo.toml").exists()
        {
            return Some(current.to_path_buf());
        }
        current = current.parent()?;
    }
}

/// Prepares a filtered mirror only when the managed package marker is present.
/// Chỉ chuẩn bị mirror đã lọc khi có marker package do MagiCore quản lý.
pub(in crate::bundler) fn prepare_workspace(entry: &Path) -> Result<PreparedWorkspace> {
    let working_dir = detect_working_dir(entry)
        .unwrap_or_else(|| entry.parent().unwrap_or(Path::new(".")).to_path_buf());
    let node_modules = working_dir.join("node_modules");
    if !node_modules.exists() {
        return Ok(PreparedWorkspace {
            entry: entry.to_path_buf(),
            working_dir,
            _temp_root: None,
        });
    }

    let needs_materialized_resolver = node_modules.join(".magicore").exists();
    if !needs_materialized_resolver {
        return Ok(PreparedWorkspace {
            entry: entry.to_path_buf(),
            working_dir,
            _temp_root: None,
        });
    }

    let temp_parent = working_dir.join(".magicore");
    std::fs::create_dir_all(&temp_parent)?;
    let temp_guard = tempfile::Builder::new()
        .prefix("tmp-build-workspace-")
        .tempdir_in(&temp_parent)?;
    let temp_root = temp_guard.path().to_path_buf();
    mirror_project_for_build(&working_dir, &temp_root)?;
    let mirrored_node_modules = temp_root.join("node_modules");
    link_node_modules_for_build(&node_modules, &mirrored_node_modules)?;
    let relative_entry = entry.strip_prefix(&working_dir)?.to_path_buf();

    Ok(PreparedWorkspace {
        entry: temp_root.join(relative_entry),
        working_dir: temp_root,
        _temp_root: Some(temp_guard),
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

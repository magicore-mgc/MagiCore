//! Project discovery and temporary mirror for Web builds.
//! Tìm dự án và tạo mirror tạm cho build Web.

use anyhow::Result;
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

/// Adds a platform-native directory symlink at the mirrored workspace path.
/// Tạo symlink thư mục theo nền tảng tại path workspace mirror.
fn link_node_modules_for_build(source: &Path, target: &Path) -> Result<()> {
    if target.exists() {
        return Ok(());
    }

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }

    symlink_dir(source, target)
        .map_err(|error| super::super::error::link_node_modules_failed(source, target, &error))
}

#[cfg(unix)]
fn symlink_dir(source: &Path, target: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(source, target)
}

#[cfg(windows)]
fn symlink_dir(source: &Path, target: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(source, target)
}

/// Copies regular project files while preserving the historical skip policy.
/// Copy file thường của dự án và giữ policy bỏ qua path hiện hành.
fn mirror_project_for_build(source_root: &Path, target_root: &Path) -> Result<()> {
    std::fs::create_dir_all(target_root)?;
    for entry in walkdir::WalkDir::new(source_root) {
        let entry = entry?;
        let path = entry.path();
        if path == source_root {
            continue;
        }

        let relative = path.strip_prefix(source_root)?;
        if should_skip_project_path(relative) {
            continue;
        }

        let target_path = target_root.join(relative);
        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&target_path)?;
            continue;
        }

        if !entry.file_type().is_file() {
            continue;
        }

        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        match std::fs::hard_link(path, &target_path) {
            Ok(()) => {}
            Err(_) => {
                std::fs::copy(path, &target_path)?;
            }
        }
    }
    Ok(())
}

/// Excludes package stores, generated output and MagiCore's own state.
/// Bỏ package store, output sinh ra và state nội bộ MagiCore.
fn should_skip_project_path(relative: &Path) -> bool {
    let mut components = relative.components();
    let Some(first) = components.next() else {
        return false;
    };
    let first = first.as_os_str().to_string_lossy();
    matches!(
        first.as_ref(),
        "node_modules" | ".magicore" | "dist" | "build" | ".next" | "out"
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

//! Mirror a project tree for bundling and link node_modules.
//! Mirror cây project để bundle và tạo link node_modules.

use anyhow::Result;
use std::path::Path;

/// Adds a platform-native directory symlink at the mirrored workspace path.
/// Tạo symlink thư mục theo nền tảng tại path workspace mirror.
pub(crate) fn link_node_modules_for_build(source: &Path, target: &Path) -> Result<()> {
    if target.exists() {
        return Ok(());
    }

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }

    symlink_dir(source, target)
        .map_err(|error| crate::bundler::error::link_node_modules_failed(source, target, &error))
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
pub(crate) fn mirror_project_for_build(source_root: &Path, target_root: &Path) -> Result<()> {
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

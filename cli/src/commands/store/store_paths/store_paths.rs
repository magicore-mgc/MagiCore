//! Resolves project-local store, lock and virtual-store paths.
//! Phân giải path store, lock, virtual-store của project.

use anyhow::Result;
use std::path::{Path, PathBuf};

pub(in crate::commands::store) fn store_db_for(project_root: &Path) -> Result<PathBuf> {
    Ok(store_cache_root_for(project_root).join("store.db"))
}

/// Web installation's `Layout::root()` for this project; this is also the lock key.
/// `Layout::root()` của Web install cho project này; đây cũng là lock key.
pub(in crate::commands::store) fn store_cache_root_for(project_root: &Path) -> PathBuf {
    project_root.join(".magicore").join("cache").join("web")
}

/// Web installation's per-project lock directory.
/// Thư mục lock theo project mà Web installation sử dụng.
pub(in crate::commands::store) fn locks_dir_for(project_root: &Path) -> std::path::PathBuf {
    store_cache_root_for(project_root).join("locks")
}

pub(in crate::commands::store) fn vstore_root_for(project_root: &Path) -> PathBuf {
    project_root.join("node_modules").join(".magicore")
}

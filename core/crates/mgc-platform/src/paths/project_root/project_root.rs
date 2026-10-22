//! Walk up from a starting path until a project marker is found.
//! Đi ngược từ path bắt đầu tới khi gặp marker project.

use std::path::{Path, PathBuf};

/// Detect project root (walk up for markers) — shared with mgc-config/paths.rs
pub fn find_project_root(from: &Path) -> Option<PathBuf> {
    let mut current = from.to_path_buf();
    loop {
        if current.join("mgc.toml").exists()
            || current.join("mgc.lock").exists()
            || current.join("package.json").exists()
            || current.join("Cargo.toml").exists()
            || current.join("pyproject.toml").exists()
            || current.join(".git").exists()
        {
            return Some(current);
        }
        if !current.pop() {
            break;
        }
    }
    None
}

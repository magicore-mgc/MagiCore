//! `cache_dir.rs` — Web project cache location.
//! đường dẫn cache Web theo project

use std::path::{Path, PathBuf};

pub fn project_cache_dir(project_root: &Path) -> PathBuf {
    project_root.join(".magicore").join("cache").join("web")
}

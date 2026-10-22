//! Resolve the project-local or user-global patch metadata directory.
//! Tìm thư mục metadata patch theo dự án hoặc theo người dùng.

use super::error::PatchesDirectoryError;
use anyhow::Result;
use mgc_platform::paths::{GlobalPaths, ProjectPaths};
use std::path::{Path, PathBuf};

/// Get patches directory from project or global
/// Lấy thư mục patch theo dự án hoặc cấu hình toàn cục.
pub fn get_patches_dir(project_root: Option<&Path>) -> Result<PathBuf> {
    if let Some(root) = project_root {
        let paths = ProjectPaths::from_root(root);
        Ok(paths.patches_dir().to_path_buf())
    } else {
        let paths = GlobalPaths::new().map_err(PatchesDirectoryError::from)?;
        Ok(paths.patches_dir().to_path_buf())
    }
}

#[cfg(test)]
#[path = "test/patches_dir.rs"]
mod tests;

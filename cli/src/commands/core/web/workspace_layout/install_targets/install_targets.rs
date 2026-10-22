//! Expands detected project mode to the exact install target list.
//! Chuyển mode project đã nhận diện thành danh sách target cài chính xác.

use anyhow::Result;
use std::path::{Path, PathBuf};

use super::super::project_mode::detect_project_mode;
use super::super::types::WebProjectMode;

pub(in crate::commands::core::web) fn install_targets(project_root: &Path) -> Result<Vec<PathBuf>> {
    match detect_project_mode(project_root)? {
        WebProjectMode::Standalone => Ok(vec![project_root.to_path_buf()]),
        WebProjectMode::FullstackSplit => Ok(vec![
            project_root.to_path_buf(),
            project_root.join("server"),
        ]),
        WebProjectMode::Monorepo => discover_monorepo_install_targets(project_root),
    }
}

fn discover_monorepo_install_targets(project_root: &Path) -> Result<Vec<PathBuf>> {
    mgc_workspace::discover_workspace_targets(project_root)
}

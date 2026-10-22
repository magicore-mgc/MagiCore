//! Selects the package/native project directory used by `mgc dev`.
//! Chọn thư mục package/native project cho `mgc dev`.

use anyhow::Result;
use std::path::{Path, PathBuf};

use super::super::super::{infer_native_dev_launch, read_dev_script};
use super::super::project_mode::workspace_frontend_dir;
use crate::commands::core::web::implementation::project_context::project_root::install_hint_command;

pub(in crate::commands::core::web) fn detect_dev_target(project_root: &Path) -> Result<PathBuf> {
    let root_script = read_dev_script(project_root)?;
    if root_script
        .as_deref()
        .is_some_and(|script| !script.starts_with("mgc "))
    {
        return Ok(project_root.to_path_buf());
    }

    let frontend = workspace_frontend_dir(project_root)?;
    if frontend.join("package.json").exists() {
        return Ok(frontend);
    }

    if root_script.is_some() {
        return Ok(project_root.to_path_buf());
    }

    if infer_native_dev_launch(project_root, None, None).is_ok() {
        return Ok(project_root.to_path_buf());
    }

    Err(crate::error::web_no_dev_target(
        project_root,
        install_hint_command(),
    ))
}

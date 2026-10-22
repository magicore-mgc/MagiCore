//! Hardware project-root creation for init.
//! Tạo root dự án Hardware cho lệnh init.

use crate::wizard::engine::ScaffoldConfig;
use anyhow::Result;
use std::path::PathBuf;

pub(in crate::commands::init) async fn init_hardware(config: &ScaffoldConfig) -> Result<PathBuf> {
    let root = std::env::current_dir()
        .map_err(|e| crate::error::cwd_deleted(&e))?
        .join(&config.project_name);
    std::fs::create_dir_all(&root)?;
    let framework = config
        .frameworks
        .first()
        .map(|s| s.as_str())
        .unwrap_or("optimizer");
    crate::commands::core::hardware::materialize_template(&root, framework).await?;
    Ok(root)
}

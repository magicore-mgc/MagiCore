use super::cloud_type::cloud_type;
use super::{native_deploy_command, native_dev_command};
use anyhow::Result;
use std::path::PathBuf;

fn project_root() -> Result<PathBuf> {
    crate::commands::core::shared::core_project_root("clo")
}

pub async fn dev(dry_run: bool) -> Result<()> {
    let root = project_root()?;
    let kind = cloud_type(&root)?;
    let _ = (dry_run, &root);
    native_dev_command(&kind)?;
    Ok(())
}

/// `mgc deploy` — mặc định dry-run (in lệnh deploy theo type, KHÔNG chạy);
/// chạy thật chỉ với `--run` (spec §4: deploy = hành động ghi cloud).
pub async fn deploy(run: bool) -> Result<()> {
    let root = project_root()?;
    let kind = cloud_type(&root)?;
    let _ = (run, &root);
    native_deploy_command(&kind)?;
    Ok(())
}

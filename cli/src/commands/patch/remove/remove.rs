//! Remove a configured package patch. / Xóa patch package đã cấu hình.

use anyhow::{Result, bail};
use mgc_config::project::ProjectConfig;
use std::path::Path;

pub(in crate::commands::patch) async fn remove_patch(
    project: &mut ProjectConfig,
    project_root: &Path,
    package: &str,
) -> Result<()> {
    let len_before = project.patches.len();
    project.patches.retain(|p| p.package != package);
    if project.patches.len() == len_before {
        bail!("No patch found for package: {}", package);
    }
    project.save(project_root)?;
    println!("Removed patch for {}", package);
    Ok(())
}

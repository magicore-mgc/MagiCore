//! List configured patches. / Liệt kê patch đã cấu hình.

use anyhow::Result;
use mgc_config::project::ProjectConfig;

pub(in crate::commands::patch) async fn list_patches(project: &ProjectConfig) -> Result<()> {
    if project.patches.is_empty() {
        println!("No patches configured");
        return Ok(());
    }
    println!("Configured patches (not applied by install):");
    for p in &project.patches {
        println!(
            "  {} @ {} -> {} ({})",
            p.package, p.version_range, p.patch_path, p.integrity
        );
    }
    Ok(())
}

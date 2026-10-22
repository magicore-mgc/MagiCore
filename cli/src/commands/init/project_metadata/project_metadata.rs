//! Project configuration, attestation, and core-specific script output.
//! Ghi cấu hình dự án, attestation và script tùy theo core.

use crate::scaffold::Scaffolder;
use crate::wizard::engine::ScaffoldConfig;
use anyhow::Result;
use mgc_config::project::ProjectConfig;
use std::path::Path;

pub(in crate::commands::init) fn write_mgc_toml(
    project_dir: &Path,
    config: &ScaffoldConfig,
) -> Result<()> {
    let name = Scaffolder::display_name(project_dir);
    let template = config.template_dir.to_str().unwrap_or("").to_string();
    let proj_config = ProjectConfig::from_scaffold(
        name,
        &config.core,
        &config.sub_type,
        config.frameworks.clone(),
        template,
        config.features.clone(),
    );
    proj_config.save(project_dir)?;
    // Best-effort core attestation: binds (.mgc.core + mgc.toml) to a home
    // key; never fails project creation (the read path warns on use).
    // (Chứng thực core best-effort: không bao giờ làm hỏng tạo project.)
    crate::commands::trust::anchor::attest_new_project(project_dir, &config.core);
    if config.core == "game" && config.frameworks.iter().any(|name| name == "bevy") {
        // Cargo scripts currently apply only to the Bevy scaffold — không gắn lệnh Rust cho engine khác.
        let scripts = "\n[scripts]\nrun = \"cargo run\"\nbuild = \"cargo build\"\n".to_string();
        let path = project_dir.join("mgc.toml");
        let mut content = std::fs::read_to_string(&path)?;
        content.push_str(&scripts);
        std::fs::write(path, content)?;
    }
    Ok(())
}

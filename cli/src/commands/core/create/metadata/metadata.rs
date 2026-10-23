//! Write project metadata after a successful scaffold.
//! Ghi metadata project sau khi scaffold thành công.

use anyhow::Result;
use mgc_config::project::ProjectConfig;
use std::path::{Path, PathBuf};

pub(crate) fn save_scaffold_metadata(
    project_dir: &Path,
    config: &crate::wizard::engine::ScaffoldConfig,
) -> Result<()> {
    let name = crate::scaffold::processor::Scaffolder::display_name(project_dir);
    let template = config.template_dir.to_str().unwrap_or("").to_string();
    let project = ProjectConfig::from_scaffold(
        name,
        &config.core,
        &config.sub_type,
        config.frameworks.clone(),
        template,
        config.features.clone(),
    );
    project.save(project_dir)?;
    Ok(())
}

pub(crate) fn scaffold_and_save_metadata(
    config: &crate::wizard::engine::ScaffoldConfig,
) -> Result<PathBuf> {
    let project_dir = crate::scaffold::processor::Scaffolder::scaffold(config)?;
    save_scaffold_metadata(&project_dir, config)?;
    Ok(project_dir)
}

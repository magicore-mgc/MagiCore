use anyhow::Result;
use mgc_config::project::ProjectConfig;
use std::fs;
use std::path::Path;

pub(in crate::commands::publish) fn commit_project_version(
    project_root: &Path,
    project: &ProjectConfig,
    pkg_json_path: &Path,
    pkg_json: &serde_json::Value,
) -> Result<()> {
    // Save both files under one writer lock so a concurrent mutation cannot interleave.
    let _guard = mgc_lockfile::project_lock::ProjectWriteLock::acquire(
        project_root,
        crate::commands::core::shared::writer_lock_timeout(project_root),
    )
    .map_err(|e| anyhow::anyhow!("publish cannot acquire the project writer lock: {e}"))?;
    project.save(project_root)?;
    if pkg_json_path.exists() {
        fs::write(pkg_json_path, serde_json::to_string_pretty(pkg_json)?)?;
    }

    Ok(())
}

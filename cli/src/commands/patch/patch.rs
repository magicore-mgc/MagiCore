//! Patch command orchestration. / Điều phối lệnh patch.

use anyhow::Result;
use mgc_config::project::ProjectConfig;
use std::path::Path;

use super::types::{PatchArgs, PatchCmd};

pub async fn run(args: PatchArgs) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root =
        ProjectConfig::find_project_root(&cwd).ok_or_else(crate::error::project_root_missing)?;
    run_at_project_root(args, &project_root).await
}

pub(super) async fn run_at_project_root(args: PatchArgs, project_root: &Path) -> Result<()> {
    match args.cmd {
        PatchCmd::Add {
            package,
            file,
            range,
        } => {
            let lock = acquire_patch_mutation_lock(project_root)?;
            let mut project =
                ProjectConfig::load(project_root)?.ok_or_else(crate::error::mgc_toml_missing)?;
            super::add::add_patch(&mut project, project_root, &package, &file, range, &lock).await
        }
        PatchCmd::Remove { package } => {
            let _lock = acquire_patch_mutation_lock(project_root)?;
            let mut project =
                ProjectConfig::load(project_root)?.ok_or_else(crate::error::mgc_toml_missing)?;
            super::remove::remove_patch(&mut project, project_root, &package).await
        }
        PatchCmd::List => {
            let project =
                ProjectConfig::load(project_root)?.ok_or_else(crate::error::mgc_toml_missing)?;
            super::list::list_patches(&project).await
        }
        PatchCmd::Verify => {
            let project =
                ProjectConfig::load(project_root)?.ok_or_else(crate::error::mgc_toml_missing)?;
            super::verify::verify_patches(&project, project_root).await
        }
    }
}

fn acquire_patch_mutation_lock(
    project_root: &Path,
) -> Result<mgc_lockfile::project_lock::ProjectWriteLock> {
    let lock = mgc_lockfile::project_lock::ProjectWriteLock::acquire(
        project_root,
        crate::commands::core::shared::writer_lock_timeout(project_root),
    )
    .map_err(|error| anyhow::anyhow!("patch cannot acquire the project writer lock: {error}"))?;
    crate::commands::core::shared::ensure_no_pending_remove_journal(project_root, &lock)?;
    Ok(lock)
}

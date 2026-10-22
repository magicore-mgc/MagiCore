//! Creates a project Web-store backup through no-follow owners.
//! Tạo backup Web store của project qua owner no-follow.

use std::path::{Path, PathBuf};

use super::super::backup_storage::{default_backup_dir, validate_present_tree};
use super::destination_validation::{prepare_backup_destination, validate_backup_destination};
use super::error::BackupError;
use super::payload_copy::copy_backup_payload;
use super::source_snapshot::{ProjectSources, inspect_project_sources};

pub(in crate::commands::store) fn run_backup(
    project_root: &Path,
    path: Option<String>,
) -> anyhow::Result<()> {
    backup_inner(project_root, path).map_err(anyhow::Error::new)
}

fn backup_inner(project_root: &Path, path: Option<String>) -> Result<(), BackupError> {
    let uses_default_backup_path = path.is_none();
    let requested_backup_dir = match path {
        Some(path) => PathBuf::from(path),
        None => default_backup_dir().map_err(BackupError::DefaultBackupPath)?,
    };
    let sources: ProjectSources = inspect_project_sources(project_root)?;

    let backup_candidate = validate_backup_destination(
        &requested_backup_dir,
        sources.cache_root.as_deref(),
        sources.vstore_source.as_deref(),
    )?;

    let validated_vstore = sources
        .vstore_source
        .as_deref()
        .map(validate_present_tree)
        .transpose()
        .map_err(|source| BackupError::TreeOperation {
            operation: "validate the project virtual store before backup",
            source,
        })?;

    let backup_root = prepare_backup_destination(
        &requested_backup_dir,
        uses_default_backup_path,
        &backup_candidate,
        sources.cache_root.as_deref(),
        sources.vstore_source.as_deref(),
    )?;

    let payload = copy_backup_payload(
        &backup_root,
        sources.database_source.as_deref(),
        validated_vstore.as_ref(),
    )?;

    println!("backup store → {}", requested_backup_dir.display());
    println!("  store.db: {} bytes", payload.database_bytes);
    println!("  vstore logical payload: {} bytes", payload.vstore_bytes);
    println!(
        "  restore: mgc store restore --path {}",
        requested_backup_dir.display()
    );
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

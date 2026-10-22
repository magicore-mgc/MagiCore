//! Coordinates validation, state preservation and project restore.
//! Điều phối xác thực, bảo toàn state và restore project.

use std::path::Path;

use super::backup_validation::{BackupValidationError, ValidatedBackup, validate_backup};
use super::database_restore::{DatabaseRestoreError, restore_database};
use super::error::RestoreError;
use super::preserve_state::{PreserveStateError, preserve_current_state};
use super::project_state::{ProjectState, inspect_project_state, validate_restore_source};
use super::safety_destination::prepare_safety_backup;
use super::virtual_store_restore::{VirtualStoreRestoreError, restore_virtual_store};
use crate::commands::store::acquire_maintenance_lock;

pub(in crate::commands::store) fn run_restore(
    project_root: &Path,
    path: String,
) -> anyhow::Result<()> {
    restore_inner(project_root, Path::new(&path), None).map_err(anyhow::Error::new)
}

fn restore_inner(
    project_root: &Path,
    backup_path: &Path,
    safety_backup_override: Option<&Path>,
) -> Result<(), RestoreError> {
    let backup: ValidatedBackup =
        validate_backup(backup_path).map_err(|source: BackupValidationError| {
            RestoreError::ValidateBackup {
                source: anyhow::Error::new(source),
            }
        })?;

    // Serialize every restore mutation with Web install, backup and store prune.
    // Tuần tự hóa mọi mutation restore với Web install, backup và store prune.
    let _project_lock = acquire_maintenance_lock(project_root).map_err(
        |source: crate::commands::store::MaintenanceLockError| RestoreError::AcquireProjectLock {
            source: anyhow::Error::new(source),
        },
    )?;

    let project_state: ProjectState = inspect_project_state(project_root)?;
    validate_restore_source(&backup, &project_state)?;
    let safety_root = prepare_safety_backup(&backup, &project_state, safety_backup_override)?;

    preserve_current_state(
        project_state.current_database.as_deref(),
        project_state.current_vstore.as_deref(),
        &safety_root,
    )
    .map_err(
        |source: PreserveStateError| RestoreError::PreserveCurrentState {
            safety_backup: safety_root.clone(),
            source: anyhow::Error::new(source),
        },
    )?;

    if let Some(source) = backup.database.as_deref() {
        restore_database(source, &project_state.database_destination).map_err(
            |source: DatabaseRestoreError| RestoreError::RestoreDatabase {
                safety_backup: safety_root.clone(),
                source: anyhow::Error::new(source),
            },
        )?;
    }
    if let Some(source) = backup.virtual_store.as_deref() {
        restore_virtual_store(source, &project_state.virtual_store_destination).map_err(
            |source: VirtualStoreRestoreError| RestoreError::RestoreVirtualStore {
                safety_backup: safety_root.clone(),
                source: anyhow::Error::new(source),
            },
        )?;
    }

    println!("restore store ← {}", backup_path.display());
    println!("  safe backup (before restore): {}", safety_root.display());
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

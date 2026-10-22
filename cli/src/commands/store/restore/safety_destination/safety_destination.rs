//! Resolves, checks and prepares a non-overlapping pre-restore safety backup.
//! Resolve, kiểm tra và chuẩn bị safety backup không overlap trước restore.

use std::path::{Path, PathBuf};

use crate::commands::store::backup_storage::{
    default_backup_dir, prepare_destination_root, prospective_destination_root,
};
use crate::commands::store::paths_overlap;

use super::super::backup_validation::ValidatedBackup;
use super::super::error::RestoreError;
use super::super::project_state::ProjectState;

pub(in crate::commands::store::restore) fn prepare_safety_backup(
    backup: &ValidatedBackup,
    project_state: &ProjectState,
    safety_backup_override: Option<&Path>,
) -> Result<PathBuf, RestoreError> {
    let uses_default_safety_path = safety_backup_override.is_none();
    let safety_backup = match safety_backup_override {
        Some(path) => path.to_path_buf(),
        None => {
            default_backup_dir().map_err(|source| RestoreError::PrepareSafetyBackup { source })?
        }
    };
    let safety_candidate = prospective_destination_root(&safety_backup).map_err(|source| {
        RestoreError::PrepareSafetyBackup {
            source: anyhow::Error::new(source),
        }
    })?;
    if paths_overlap(&safety_candidate, &backup.root) {
        return Err(RestoreError::BackupOverlap {
            backup_root: backup.root.display().to_string(),
            safety: safety_candidate.display().to_string(),
        });
    }
    if paths_overlap(&safety_candidate, &project_state.project_database_root)
        || paths_overlap(&safety_candidate, &project_state.project_vstore_root)
    {
        return Err(RestoreError::BackupOverlap {
            backup_root: safety_candidate.display().to_string(),
            safety: project_state.canonical_project.display().to_string(),
        });
    }
    if uses_default_safety_path {
        let default_root =
            safety_backup
                .parent()
                .ok_or_else(|| RestoreError::PrepareSafetyBackup {
                    source: anyhow::anyhow!("default safety-backup path has no parent"),
                })?;
        prepare_destination_root(default_root).map_err(|source| {
            RestoreError::PrepareSafetyBackup {
                source: anyhow::Error::new(source),
            }
        })?;
    }
    let safety_root = prepare_destination_root(&safety_backup).map_err(|source| {
        RestoreError::PrepareSafetyBackup {
            source: anyhow::Error::new(source),
        }
    })?;
    if safety_root != safety_candidate {
        return Err(RestoreError::SafetyBackupDestinationChanged {
            expected: safety_candidate,
            actual: safety_root,
        });
    }
    if paths_overlap(&safety_root, &backup.root) {
        return Err(RestoreError::BackupOverlap {
            backup_root: backup.root.display().to_string(),
            safety: safety_root.display().to_string(),
        });
    }
    if paths_overlap(&safety_root, &project_state.project_database_root)
        || paths_overlap(&safety_root, &project_state.project_vstore_root)
    {
        return Err(RestoreError::BackupOverlap {
            backup_root: safety_root.display().to_string(),
            safety: project_state.canonical_project.display().to_string(),
        });
    }
    Ok(safety_root)
}

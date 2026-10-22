//! Captures current Web-store paths after the maintenance lock is held.
//! Chụp các path Web-store hiện tại sau khi đã giữ maintenance lock.

use std::path::{Path, PathBuf};

use crate::commands::safe_filesystem::canonical_directory;
use crate::commands::store::paths_overlap;
use crate::commands::store::prune::{
    existing_database_file, existing_store_cache_root, existing_vstore_root,
};
use crate::commands::store::store_paths::{store_db_for, vstore_root_for};

use super::super::backup_validation::ValidatedBackup;
use super::super::error::RestoreError;

pub(in crate::commands::store::restore) struct ProjectState {
    pub(in crate::commands::store::restore) database_destination: PathBuf,
    pub(in crate::commands::store::restore) current_database: Option<PathBuf>,
    pub(in crate::commands::store::restore) virtual_store_destination: PathBuf,
    pub(in crate::commands::store::restore) current_vstore: Option<PathBuf>,
    pub(in crate::commands::store::restore) canonical_project: PathBuf,
    pub(in crate::commands::store::restore) project_database_root: PathBuf,
    pub(in crate::commands::store::restore) project_vstore_root: PathBuf,
}

pub(in crate::commands::store::restore) fn inspect_project_state(
    project_root: &Path,
) -> Result<ProjectState, RestoreError> {
    let database_destination = store_db_for(project_root)
        .map_err(|source| RestoreError::ResolveDatabasePath { source })?;
    let current_database = existing_store_cache_root(project_root)
        .map_err(|source| RestoreError::InspectProjectState {
            source: anyhow::Error::new(source),
        })?
        .map(|root| existing_database_file(&root))
        .transpose()
        .map_err(|source| RestoreError::InspectProjectState {
            source: anyhow::Error::new(source),
        })?
        .flatten();
    let virtual_store_destination = vstore_root_for(project_root);
    let current_vstore =
        existing_vstore_root(project_root).map_err(|source| RestoreError::InspectProjectState {
            source: anyhow::Error::new(source),
        })?;
    let canonical_project =
        canonical_directory(project_root).map_err(|source| RestoreError::InspectProjectState {
            source: anyhow::Error::new(source),
        })?;
    let project_database_root = canonical_project.join(".magicore/cache/web");
    let project_vstore_root = canonical_project.join("node_modules/.magicore");

    Ok(ProjectState {
        database_destination,
        current_database,
        virtual_store_destination,
        current_vstore,
        canonical_project,
        project_database_root,
        project_vstore_root,
    })
}

pub(in crate::commands::store::restore) fn validate_restore_source(
    backup: &ValidatedBackup,
    project_state: &ProjectState,
) -> Result<(), RestoreError> {
    if backup.database.is_some()
        && paths_overlap(&backup.root, &project_state.project_database_root)
    {
        return Err(RestoreError::BackupOverlap {
            backup_root: backup.root.display().to_string(),
            safety: project_state.project_database_root.display().to_string(),
        });
    }
    if backup.virtual_store.is_some()
        && paths_overlap(&backup.root, &project_state.project_vstore_root)
    {
        return Err(RestoreError::BackupOverlap {
            backup_root: backup.root.display().to_string(),
            safety: project_state.project_vstore_root.display().to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

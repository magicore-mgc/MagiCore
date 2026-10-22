//! Captures stable project store roots while holding the maintenance lock.
//! Chụp root store ổn định trong lúc giữ maintenance lock.

use std::io::Read;
use std::path::{Path, PathBuf};

use crate::commands::safe_filesystem::canonical_directory;
use crate::commands::store::prune::{
    existing_database_file, existing_store_cache_root, existing_vstore_root,
    validate_existing_lock_directory,
};
use crate::commands::store::{MaintenanceLockError, acquire_maintenance_lock};

use super::super::error::BackupError;

pub(in crate::commands::store::backup) struct ProjectSources {
    pub(in crate::commands::store::backup) cache_root: Option<PathBuf>,
    pub(in crate::commands::store::backup) database_source: Option<PathBuf>,
    pub(in crate::commands::store::backup) vstore_source: Option<PathBuf>,
    pub(in crate::commands::store::backup) _project_lock: mgc_store::ProjectInstallLock,
}

pub(in crate::commands::store::backup) fn inspect_project_sources(
    project_root: &Path,
) -> Result<ProjectSources, BackupError> {
    let initial_project_root =
        canonical_directory(project_root).map_err(|source| BackupError::InspectProjectStore {
            operation: "canonicalize the project root",
            source: anyhow::Error::new(source),
        })?;
    let initial_cache_root = existing_store_cache_root(project_root).map_err(|source| {
        BackupError::InspectProjectStore {
            operation: "resolve the project store root",
            source: anyhow::Error::new(source),
        }
    })?;
    if let Some(root) = &initial_cache_root {
        validate_existing_lock_directory(root).map_err(|source| {
            BackupError::InspectProjectStore {
                operation: "validate the project maintenance-lock directory",
                source: anyhow::Error::new(source),
            }
        })?;
    }
    let initial_vstore_root =
        existing_vstore_root(project_root).map_err(|source| BackupError::InspectProjectStore {
            operation: "inspect the project virtual store",
            source: anyhow::Error::new(source),
        })?;

    let project_lock =
        acquire_maintenance_lock(project_root).map_err(|source: MaintenanceLockError| {
            BackupError::AcquireProjectLock {
                source: anyhow::Error::new(source),
            }
        })?;

    let cache_root = existing_store_cache_root(project_root).map_err(|source| {
        BackupError::InspectProjectStore {
            operation: "recheck the project store root after locking",
            source: anyhow::Error::new(source),
        }
    })?;
    let locked_project_root =
        canonical_directory(project_root).map_err(|source| BackupError::InspectProjectStore {
            operation: "recheck the project root after locking",
            source: anyhow::Error::new(source),
        })?;
    if locked_project_root != initial_project_root {
        return Err(BackupError::StoreRootChanged {
            path: project_root.to_path_buf(),
        });
    }
    if initial_cache_root
        .as_ref()
        .is_some_and(|initial| cache_root.as_ref() != Some(initial))
    {
        return Err(BackupError::StoreRootChanged {
            path: project_root.to_path_buf(),
        });
    }
    if let Some(root) = &cache_root {
        validate_existing_lock_directory(root).map_err(|source| {
            BackupError::InspectProjectStore {
                operation: "recheck the project maintenance-lock directory",
                source: anyhow::Error::new(source),
            }
        })?;
    }
    let database_source = match &cache_root {
        Some(root) => {
            existing_database_file(root).map_err(|source| BackupError::InspectProjectStore {
                operation: "inspect the project database after locking",
                source: anyhow::Error::new(source),
            })?
        }
        None => None,
    };
    if let Some(path) = &database_source {
        let mut database = mgc_platform::open_regular_file_no_follow(path).map_err(|source| {
            BackupError::InspectProjectStore {
                operation: "open the project database without following symlinks",
                source: anyhow::Error::new(source),
            }
        })?;
        let mut first_byte = [0_u8; 1];
        database
            .read(&mut first_byte)
            .map_err(|source| BackupError::InspectProjectStore {
                operation: "read the project database before backup",
                source: anyhow::Error::new(source),
            })?;
    }
    let vstore_source =
        existing_vstore_root(project_root).map_err(|source| BackupError::InspectProjectStore {
            operation: "recheck the project virtual store after locking",
            source: anyhow::Error::new(source),
        })?;
    if initial_vstore_root
        .as_ref()
        .is_some_and(|initial| vstore_source.as_ref() != Some(initial))
    {
        return Err(BackupError::StoreRootChanged {
            path: project_root.join("node_modules/.magicore"),
        });
    }
    if database_source.is_none() && vstore_source.is_none() {
        return Err(BackupError::EmptyProjectStore);
    }

    Ok(ProjectSources {
        cache_root,
        database_source,
        vstore_source,
        _project_lock: project_lock,
    })
}

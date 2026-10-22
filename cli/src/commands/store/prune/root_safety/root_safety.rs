//! Checks store ancestry and database files without following final symlinks.
//! Kiểm tra cây thư mục store và DB, không follow symlink cuối.

use std::fs;
use std::path::{Path, PathBuf};

use super::error::RootSafetyError;
use crate::commands::safe_filesystem::{canonical_directory, existing_directory_under};

pub(in crate::commands::store) fn existing_store_cache_root(
    project_root: &Path,
) -> Result<Option<PathBuf>, RootSafetyError> {
    let mut parent = canonical_directory(project_root)?;
    for child in [".magicore", "cache", "web"] {
        let Some(canonical_child) = existing_directory_under(&parent.join(child), &parent)? else {
            return Ok(None);
        };
        parent = canonical_child;
    }
    Ok(Some(parent))
}

pub(in crate::commands::store) fn existing_database_file(
    cache_root: &Path,
) -> Result<Option<PathBuf>, RootSafetyError> {
    let database_path = cache_root.join("store.db");
    let metadata = match fs::symlink_metadata(&database_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => {
            return Err(RootSafetyError::InspectDatabase {
                path: display_path(&database_path),
                source,
            });
        }
    };
    if metadata.file_type().is_symlink() {
        return Err(RootSafetyError::SymlinkedDatabase {
            path: display_path(&database_path),
        });
    }
    if !metadata.is_file() {
        return Err(RootSafetyError::DatabaseNotRegularFile {
            path: display_path(&database_path),
        });
    }

    let canonical = fs::canonicalize(&database_path).map_err(|source| {
        RootSafetyError::CanonicalizeDatabase {
            path: display_path(&database_path),
            source,
        }
    })?;
    if !canonical.starts_with(cache_root) {
        return Err(RootSafetyError::DatabaseEscapesCache {
            path: display_path(&canonical),
            root: display_path(cache_root),
        });
    }
    let confirmed = fs::symlink_metadata(&database_path).map_err(|source| {
        RootSafetyError::InspectDatabase {
            path: display_path(&database_path),
            source,
        }
    })?;
    if confirmed.file_type().is_symlink() {
        return Err(RootSafetyError::SymlinkedDatabase {
            path: display_path(&database_path),
        });
    }
    if !confirmed.is_file() {
        return Err(RootSafetyError::DatabaseNotRegularFile {
            path: display_path(&database_path),
        });
    }
    Ok(Some(canonical))
}

pub(in crate::commands::store) fn existing_vstore_root(
    project_root: &Path,
) -> Result<Option<PathBuf>, RootSafetyError> {
    let canonical_project = canonical_directory(project_root)?;
    let Some(node_modules) =
        existing_directory_under(&canonical_project.join("node_modules"), &canonical_project)?
    else {
        return Ok(None);
    };
    Ok(existing_directory_under(
        &node_modules.join(".magicore"),
        &node_modules,
    )?)
}

pub(in crate::commands::store) fn validate_existing_lock_directory(
    cache_root: &Path,
) -> Result<(), RootSafetyError> {
    let _ = existing_directory_under(&cache_root.join("locks"), cache_root)?;
    Ok(())
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}

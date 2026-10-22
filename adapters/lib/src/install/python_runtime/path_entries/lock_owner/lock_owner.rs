//! Read the Python-owned lock lane without re-resolving dependencies.
//! Đọc lane lock thuộc Python mà không resolve lại dependency.

use mgc_types::Manifest;
use std::path::Path;

use super::error::PythonLockOwnerError;
use crate::install::{python_project_owner, read_existing_lock};

pub(in crate::install::python_runtime::path_entries) fn read_python_lock(
    project_root: &Path,
    python_manifest: Option<&Manifest>,
) -> Result<Option<mgc_lockfile::Lockfile>, PythonLockOwnerError> {
    let lock_path = project_root.join("mgc.lock");
    let schema_v4 = mgc_config::project::lock_schema_is_v4(project_root)
        .map_err(|source| PythonLockOwnerError::SchemaDetection { source })?;
    if schema_v4 {
        return match std::fs::symlink_metadata(&lock_path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                ensure_declared_python_dependencies_are_locked(project_root, python_manifest)?;
                Ok(None)
            }
            Err(source) => Err(PythonLockOwnerError::LockfileInspection {
                path: lock_path.display().to_string(),
                source,
            }),
            Ok(_) => read_existing_lock(project_root)
                .map(Some)
                .map_err(|source| PythonLockOwnerError::V4LockSafety { source }),
        };
    }

    mgc_lockfile::parser::ensure_v3_readable_lock(&lock_path)
        .map_err(|source| PythonLockOwnerError::V3LockSafety { source })?;
    match mgc_lockfile::load_lockfile(&lock_path) {
        Ok(lockfile) => Ok(Some(lockfile)),
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            ensure_declared_python_dependencies_are_locked(project_root, python_manifest)?;
            Ok(None)
        }
        Err(source) => Err(PythonLockOwnerError::V3LockSafety { source }),
    }
}

fn ensure_declared_python_dependencies_are_locked(
    project_root: &Path,
    python_manifest: Option<&Manifest>,
) -> Result<(), PythonLockOwnerError> {
    if python_manifest.is_some_and(|manifest| !manifest.dependencies.is_empty()) {
        return Err(PythonLockOwnerError::DeclaredDependenciesWithoutLock {
            owner_core: python_project_owner(project_root)?,
        });
    }
    Ok(())
}

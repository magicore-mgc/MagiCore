//! Acquires the exact project lock used by Web install.
//! Lấy đúng project lock mà Web install đang dùng.

use std::path::Path;

use super::error::MaintenanceLockError;
use crate::commands::store::store_paths::{locks_dir_for, store_cache_root_for};

pub(in crate::commands) fn acquire_maintenance_lock(
    project_root: &Path,
) -> Result<mgc_store::ProjectInstallLock, MaintenanceLockError> {
    let locks_root = locks_dir_for(project_root);
    let project_key = store_cache_root_for(project_root)
        .to_string_lossy()
        .into_owned();
    mgc_store::ProjectInstallLock::acquire_at(&locks_root, &project_key).map_err(|source| {
        MaintenanceLockError::Acquire {
            locks_root: locks_root.display().to_string(),
            project_key,
            source,
        }
    })
}

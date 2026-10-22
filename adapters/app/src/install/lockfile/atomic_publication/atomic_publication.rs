//! Canonical lock mutation gate and atomic publication.
//! Cổng mutation lock canonical và publish nguyên tử.

use super::super::error::AppLockfileError;
#[cfg(test)]
use mgc_types::MgResult;
use std::path::Path;

/// Publish `mgc.lock` through the shared App file writer.
/// Publish `mgc.lock` qua bộ ghi file dùng chung của App.
#[cfg(test)]
pub(crate) fn atomic_write_canonical_lock(path: &Path, bytes: &[u8]) -> MgResult<()> {
    atomic_write_canonical_lock_typed(path, bytes).map_err(Into::into)
}

pub(in crate::install::lockfile) fn atomic_write_canonical_lock_typed(
    path: &Path,
    bytes: &[u8],
) -> Result<(), AppLockfileError> {
    mgc_lockfile::ensure_lockfile_mutation_allowed(path).map_err(|error| {
        AppLockfileError::caused(
            "MGC_APP_LOCK_MUTATION_DENIED",
            "authorize lock mutation",
            error.to_string(),
            error,
        )
    })?;
    crate::install::atomic_write_project_file(path, bytes, "mgc.lock").map_err(|error| {
        AppLockfileError::caused(
            "MGC_APP_LOCK_PUBLISH",
            "publish canonical lock",
            error.to_string(),
            error,
        )
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

//! Holds the project writer lock while trust-pending state is read.
//! Giữ project writer lock trong suốt thao tác đọc trạng thái trust-pending.

use mgc_lockfile::project_lock::ProjectWriteLock;
use std::path::Path;
use std::time::Duration;

use super::super::{PendingResult, TrustPendingError};

pub(in crate::commands::trust::pending) fn acquire_scan_lock(
    root: &Path,
    timeout: Duration,
) -> PendingResult<ProjectWriteLock> {
    let lock = ProjectWriteLock::acquire(root, timeout)
        .map_err(|error| TrustPendingError::lock_failed(root, error))?;
    crate::commands::core::shared::ensure_no_pending_remove_journal(root, &lock)?;
    Ok(lock)
}

#[cfg(test)]
#[path = "test/scan_lock.rs"]
mod tests;

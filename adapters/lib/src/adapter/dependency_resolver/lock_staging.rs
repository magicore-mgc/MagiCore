//! Replace pending lock rows only after native resolution succeeds.
//! Chỉ thay pending lock sau khi resolve native thành công.

use mgc_lockfile::Package;
use mgc_types::MgResult;
use std::sync::Mutex;

use super::error::DependencyResolverError;

/// Replace pending rows atomically with respect to the adapter mutex.
/// Thay toàn bộ pending row dưới mutex của adapter.
pub(in crate::adapter) fn stage_pending_lock(
    pending_lock: &Mutex<Vec<Package>>,
    packages: Vec<Package>,
) -> MgResult<()> {
    *pending_lock
        .lock()
        .map_err(|_| DependencyResolverError::PendingLockPoisoned)? = packages;
    Ok(())
}

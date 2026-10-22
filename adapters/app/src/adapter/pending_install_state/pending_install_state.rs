//! Atomic handoff state between native resolution and installation.
//! State bàn giao nguyên tử giữa resolve native và install.

use super::PendingInstallStateError;
use mgc_lockfile::Package;
use mgc_resolver::protocols::SwiftRegistryProtocol;
use mgc_types::MgResult;
use std::sync::Mutex;

#[derive(Default)]
pub(crate) struct PendingInstallState {
    inner: Mutex<PendingInstallSnapshot>,
}

#[derive(Default)]
pub(crate) struct PendingInstallSnapshot {
    pub(in crate::adapter) lock_packages: Vec<Package>,
    pub(in crate::adapter) swift_protocol: Option<SwiftRegistryProtocol>,
}

impl PendingInstallState {
    pub(in crate::adapter) fn clear(&self) -> MgResult<()> {
        *self
            .inner
            .lock()
            .map_err(|_| PendingInstallStateError::LockPoisoned)? =
            PendingInstallSnapshot::default();
        Ok(())
    }

    pub(in crate::adapter) fn store_lock(&self, lock_packages: Vec<Package>) -> MgResult<()> {
        *self
            .inner
            .lock()
            .map_err(|_| PendingInstallStateError::LockPoisoned)? = PendingInstallSnapshot {
            lock_packages,
            swift_protocol: None,
        };
        Ok(())
    }

    pub(in crate::adapter) fn store_swift(
        &self,
        lock_packages: Vec<Package>,
        swift_protocol: SwiftRegistryProtocol,
    ) -> MgResult<()> {
        *self
            .inner
            .lock()
            .map_err(|_| PendingInstallStateError::LockPoisoned)? = PendingInstallSnapshot {
            lock_packages,
            swift_protocol: Some(swift_protocol),
        };
        Ok(())
    }

    pub(in crate::adapter) fn take(&self) -> MgResult<PendingInstallSnapshot> {
        Ok(std::mem::take(
            &mut *self
                .inner
                .lock()
                .map_err(|_| PendingInstallStateError::LockPoisoned)?,
        ))
    }
}

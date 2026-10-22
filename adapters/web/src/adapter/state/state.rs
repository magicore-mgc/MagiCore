//! `state.rs` — Adapter state guards / guard state adapter.
use super::super::WebAdapter;
use mgc_types::MgResult;
use std::sync::MutexGuard;

impl WebAdapter {
    pub(in crate::adapter) fn existing_versions_guard(
        &self,
    ) -> MutexGuard<'_, std::collections::HashMap<String, String>> {
        self.existing_versions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub(in crate::adapter) fn prefetch_handle_guard(
        &self,
    ) -> MutexGuard<'_, Option<tokio::task::JoinHandle<MgResult<u64>>>> {
        self.prefetch_handle
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

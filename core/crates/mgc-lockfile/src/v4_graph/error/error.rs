//! Existing lockfile error mapping for graph refusal.
//! Ánh xạ lỗi từ chối graph vào error contract lockfile hiện có.

use crate::LockfileError;

pub(in crate::v4_graph) fn unusable_for_install(why: String) -> LockfileError {
    LockfileError::UnusableForInstall(why)
}

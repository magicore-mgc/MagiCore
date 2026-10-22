//! Wire the Windows system snapshot test seam.
//! Nối test seam chụp process table Windows.

#[path = "system_snapshot.rs"]
mod system_snapshot;
#[cfg(all(windows, test))]
pub(super) use system_snapshot::find_forbidden_descendant_with_timeout;

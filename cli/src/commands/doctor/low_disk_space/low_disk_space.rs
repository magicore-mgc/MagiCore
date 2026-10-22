//! Produces advice for low free space without claiming project prune frees global data.
//! Tạo hướng xử lý khi disk thấp, không nói prune project sẽ dọn global store.

use std::path::Path;

pub(in crate::commands::doctor) const LOW_DISK_WARNING_THRESHOLD_BYTES: u64 = 500 * 1024 * 1024;

pub(in crate::commands::doctor) struct LowDiskAdvice {
    pub(in crate::commands::doctor) message: String,
    pub(in crate::commands::doctor) suggested_action: String,
}

pub(in crate::commands::doctor) fn advice(
    free_bytes: u64,
    configured_store: &Path,
    queried_volume_path: &Path,
) -> Option<LowDiskAdvice> {
    if free_bytes == 0 || free_bytes >= LOW_DISK_WARNING_THRESHOLD_BYTES {
        return None;
    }

    let free_mib = free_bytes / (1024 * 1024);
    let store_path = configured_store.display().to_string();
    let volume_path = queried_volume_path.display().to_string();
    Some(LowDiskAdvice {
        message: format!(
            "Low disk space: only {free_mib} MiB free at existing directory '{volume_path}', used to estimate capacity for configured MagiCore store '{store_path}'."
        ),
        suggested_action: format!(
            "Use your OS disk-usage tools to inspect the volume at '{volume_path}' for configured MagiCore store '{store_path}'."
        ),
    })
}

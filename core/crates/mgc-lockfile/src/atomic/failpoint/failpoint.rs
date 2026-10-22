//! Defines deterministic crash points for the lock publication transaction.
//! Định nghĩa điểm dừng crash xác định cho transaction publish lock.

use std::time::Duration;

/// Ordered lock-write crash phases shared with the crash-recovery harness.
/// Danh sách phase ghi lock theo đúng thứ tự transaction.
pub const LOCK_FAILPOINTS: &[&str] = &[
    "lock-before-temp-write",
    "lock-after-temp-fsync",
    "lock-before-rename",
    "lock-after-rename",
    "lock-after-dir-fsync",
];

/// Park the process when its requested phase matches the active failpoint.
/// Dừng process ở phase đã chọn để harness có thể kiểm thử crash recovery.
pub fn lock_failpoint(phase: &str) {
    let Some(target) = std::env::var("MGC_LOCK_FAILPOINT")
        .ok()
        .filter(|value| !value.trim().is_empty())
    else {
        return;
    };
    if target == phase {
        if let Ok(path) = std::env::var("MGC_LOCK_FAILPOINT_READY_FILE")
            && let Ok(mut file) = std::fs::File::create(&path)
        {
            use std::io::Write;
            let _ = file.write_all(format!("READY:{phase}\n").as_bytes());
            let _ = file.sync_all();
        }
        loop {
            std::thread::sleep(Duration::from_secs(3600));
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

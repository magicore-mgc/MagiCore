//! Defines deterministic crash points for the lock publication transaction.
//! Định nghĩa điểm dừng crash xác định cho transaction publish lock.

#[cfg(feature = "fault-injection")]
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
///
/// Compiled only with the `fault-injection` feature. A default build has no
/// code path that can park on an environment variable, so an inherited
/// `MGC_LOCK_FAILPOINT` (a stray `.env`, a CI template) cannot hang a real
/// `mgc install`. The crash-recovery harness builds with the feature on.
/// (Chỉ biên dịch với feature `fault-injection`. Build mặc định không có đường
/// nào dừng theo env, nên `MGC_LOCK_FAILPOINT` lạc (từ `.env`, template CI)
/// không thể treo `mgc install` thật. Harness crash-recovery build kèm feature.)
#[cfg(feature = "fault-injection")]
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
        park_forever();
    }
}

/// Block without burning CPU until the harness kills the process.
///
/// `thread::park_timeout` in a long loop keeps the thread off the scheduler
/// instead of waking every second, but a plain long sleep is enough here and
/// avoids a spurious-wakeup spin.
/// (Chặn mà không đốt CPU tới khi harness kill process.)
#[cfg(feature = "fault-injection")]
fn park_forever() -> ! {
    loop {
        std::thread::sleep(Duration::from_secs(3600));
    }
}

/// No-op in a default build: the facade stays callable, nothing can park.
/// No-op trong build mặc định: facade vẫn gọi được, không gì có thể dừng process.
#[cfg(not(feature = "fault-injection"))]
pub fn lock_failpoint(_phase: &str) {}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

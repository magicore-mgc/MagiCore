//! Owns lock-temp naming, process-liveness proof, and conservative cleanup.
//! Quản lý tên temp lock, chứng minh process còn sống và dọn file thận trọng.

use std::path::Path;
use std::time::{Duration, SystemTime};

/// Build a unique `mgc.lock.tmp.<pid>.<time>.<counter>` candidate.
/// Tạo tên ứng viên duy nhất theo pid, thời gian và bộ đếm process-local.
pub(in crate::atomic) fn temp_name() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NONCE: AtomicU64 = AtomicU64::new(0);
    let nonce = NONCE.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("mgc.lock.tmp.{}.{nanos:x}.{nonce}", std::process::id())
}

/// Parse the process id encoded in an MGC lock temporary-file name.
/// Đọc pid được mã hóa trong tên file tạm lock của MGC.
fn temp_pid(file_name: &str) -> Option<u32> {
    let rest = file_name.strip_prefix("mgc.lock.tmp.")?;
    rest.split('.').next()?.parse().ok()
}

/// Return true only when the operating system proves that the process is dead.
/// Chỉ trả true khi hệ điều hành chứng minh process đã chết.
#[cfg(unix)]
#[allow(unsafe_code)]
fn pid_is_dead(pid: u32) -> bool {
    // SAFETY: kill(pid, 0) reports process liveness without sending a signal or touching memory.
    // AN TOÀN: kill(pid, 0) chỉ kiểm tra process, không gửi signal và không truy cập bộ nhớ.
    let result = unsafe { libc::kill(pid as libc::pid_t, 0) };
    if result == 0 {
        return false;
    }
    std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
}

/// Unknown process liveness is treated as alive on platforms without a proof primitive.
/// Nếu platform không chứng minh được thì coi process vẫn sống.
#[cfg(not(unix))]
fn pid_is_dead(_pid: u32) -> bool {
    false
}

/// Remove stale `mgc.lock.tmp.*` files older than `grace` whose owner pid is provably dead.
/// Chỉ xóa file `mgc.lock.tmp.*` cũ hơn `grace` và có pid chủ sở hữu được chứng minh đã chết.
///
/// Call only while holding the project writer lock. Returns the number removed.
/// Chỉ gọi khi đang giữ project writer lock. Trả về số file đã xóa.
pub fn cleanup_stale_temps(dir: &Path, grace: Duration) -> usize {
    let mut removed = 0;
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let now = SystemTime::now();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.starts_with("mgc.lock.tmp.") {
            continue;
        }
        let stale_time = entry
            .metadata()
            .and_then(|m| m.modified())
            .ok()
            .and_then(|mtime| now.duration_since(mtime).ok())
            .is_some_and(|age| age >= grace);
        if !stale_time {
            continue;
        }
        let pid_dead = temp_pid(&name).is_some_and(pid_is_dead);
        if !pid_dead {
            continue;
        }
        if std::fs::remove_file(entry.path()).is_ok() {
            removed += 1;
        }
    }
    removed
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

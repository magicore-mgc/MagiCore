//! Generates collision-reduced temporary paths for CAS staging callers.
//! Sinh đường dẫn tạm giảm va chạm cho caller staging CAS.

use std::path::{Path, PathBuf};

/// Unique temp path for atomic CAS writes — never a fixed name (concurrent
/// writers must not collide or clobber each other's temp files).
/// Đường dẫn temp duy nhất cho ghi CAS nguyên tử — không bao giờ tên cố định
/// (nhiều writer song song không đụng độ hay đè temp của nhau).
pub fn unique_tmp_path(dir: &Path, prefix: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let tid = {
        let tid = std::thread::current().id();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&tid, &mut hasher);
        std::hash::Hasher::finish(&hasher)
    };
    dir.join(format!(
        "{prefix}-{}-{tid:x}-{nanos}-{count}",
        std::process::id()
    ))
}

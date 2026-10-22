use crate::bench_matrix::baseline::baseline_path;
// Baseline lock lifecycle
// Tạo, chờ, phát hiện lock cũ và giải phóng lock baseline.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn acquire_baseline_lock(name: &str) -> anyhow::Result<()> {
    let lock_path = baseline_lock_path(name);
    if let Some(parent) = lock_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let max_attempts = 10usize;
    let retry_delay = std::time::Duration::from_millis(200);
    for attempt in 0..max_attempts {
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)
        {
            Ok(_file) => {
                return Ok(());
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                if is_stale_lock(&lock_path) {
                    let _ = std::fs::remove_file(&lock_path);
                    continue;
                }
                if attempt + 1 < max_attempts {
                    std::thread::sleep(retry_delay);
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("failed to acquire baseline lock: {e}"));
            }
        }
    }
    Err(anyhow::anyhow!(
        "baseline '{name}' is locked by another process (lock exists at {})",
        lock_path.display()
    ))
}

pub(in crate::bench_matrix) fn release_baseline_lock(name: &str) -> anyhow::Result<()> {
    let lock_path = baseline_lock_path(name);
    let _ = std::fs::remove_file(&lock_path);
    Ok(())
}

pub(in crate::bench_matrix) fn baseline_lock_path(name: &str) -> PathBuf {
    let mut path = baseline_path(name);
    path.set_extension("lock");
    path
}

pub(in crate::bench_matrix) fn is_stale_lock(lock_path: &Path) -> bool {
    std::fs::metadata(lock_path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.elapsed().ok())
        .is_some_and(|age| age.as_secs() > 60)
}

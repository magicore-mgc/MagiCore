//! `atomic_write.rs` — same-directory staging and synced lock replacement.
//! staging cùng thư mục và thay lock sau khi sync

use super::super::LockfileError;
use mgc_types::MgResult;
use std::path::Path;

/// Atomically publish the unified lockfile. The same-directory staging
/// file prevents crashes from leaving a truncated lock visible to other
/// ecosystem adapters.
/// (Ghi lockfile hợp nhất nguyên tử để crash không để lại file cụt.)
pub(crate) fn atomic_write_web_lockfile(path: &Path, bytes: &[u8]) -> MgResult<()> {
    mgc_lockfile::ensure_lockfile_mutation_allowed(path)
        .map_err(|error| LockfileError::Mutation(error.to_string()))?;
    use std::io::Write;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static NONCE: AtomicU64 = AtomicU64::new(0);
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            LockfileError::AtomicWrite("mgc.lock path has no valid filename".to_string())
        })?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| LockfileError::AtomicWrite(format!("system clock is invalid: {error}")))?
        .as_nanos();
    let nonce = NONCE.fetch_add(1, Ordering::Relaxed);
    let temp = parent.join(format!(
        ".{filename}.web.{}.{}.{nonce}.tmp",
        std::process::id(),
        timestamp
    ));

    let result: Result<(), LockfileError> = (|| {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW);
        }
        let mut file = options.open(&temp).map_err(|error| {
            LockfileError::AtomicWrite(format!("cannot create lock staging file: {error}"))
        })?;
        file.write_all(bytes).map_err(|error| {
            LockfileError::AtomicWrite(format!("cannot write lock staging file: {error}"))
        })?;
        file.sync_all().map_err(|error| {
            LockfileError::AtomicWrite(format!("cannot sync lock staging file: {error}"))
        })?;
        mgc_lockfile::atomic::atomic_replace_file(&temp, path).map_err(|error| {
            LockfileError::AtomicWrite(format!("cannot atomically replace mgc.lock: {error}"))
        })?;
        #[cfg(unix)]
        std::fs::File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| {
                LockfileError::AtomicWrite(format!("cannot sync lock directory: {error}"))
            })?;
        Ok(())
    })();

    if let Err(write_error) = result {
        match std::fs::remove_file(&temp) {
            Ok(()) => Err(write_error.into()),
            Err(cleanup_error) if cleanup_error.kind() == std::io::ErrorKind::NotFound => {
                Err(write_error.into())
            }
            Err(cleanup_error) => Err(LockfileError::AtomicWrite(format!(
                "{write_error}; additionally failed to remove staging file '{}': {cleanup_error}",
                temp.display()
            ))
            .into()),
        }
    } else {
        Ok(())
    }
}

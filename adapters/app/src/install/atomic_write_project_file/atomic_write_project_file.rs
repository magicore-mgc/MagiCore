//! Same-directory durable atomic file publication for App install outputs.
//! Ghi file atomic bền vững cùng thư mục cho output install App.
use mgc_types::{MgError, MgResult};
use std::path::Path;

pub(crate) fn atomic_write_project_file(path: &Path, bytes: &[u8], label: &str) -> MgResult<()> {
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
        .ok_or_else(|| MgError::Other(format!("{label} path has no valid filename")))?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| MgError::Other(format!("system clock is invalid: {e}")))?
        .as_nanos();
    let nonce = NONCE.fetch_add(1, Ordering::Relaxed);
    let temp = parent.join(format!(
        ".{filename}.{}.{}.{nonce}.tmp",
        std::process::id(),
        timestamp
    ));

    let result = (|| {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW);
        }
        let mut file = options
            .open(&temp)
            .map_err(|e| MgError::Other(format!("cannot create {label} staging file: {e}")))?;
        file.write_all(bytes)
            .map_err(|e| MgError::Other(format!("cannot write {label} staging file: {e}")))?;
        file.sync_all()
            .map_err(|e| MgError::Other(format!("cannot sync {label} staging file: {e}")))?;
        mgc_lockfile::atomic::atomic_replace_file(&temp, path)
            .map_err(|e| MgError::Other(format!("cannot atomically replace {label}: {e}")))?;
        #[cfg(unix)]
        std::fs::File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|e| MgError::Other(format!("cannot sync {label} parent directory: {e}")))?;
        Ok(())
    })();
    if let Err(write_error) = result {
        match std::fs::remove_file(&temp) {
            Ok(()) => Err(write_error),
            Err(cleanup_error) if cleanup_error.kind() == std::io::ErrorKind::NotFound => {
                Err(write_error)
            }
            Err(cleanup_error) => Err(MgError::Other(format!(
                "{write_error}; additionally failed to remove staging file '{}': {cleanup_error}",
                temp.display()
            ))),
        }
    } else {
        result
    }
}

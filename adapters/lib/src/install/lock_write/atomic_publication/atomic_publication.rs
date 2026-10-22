//! Durably stage and atomically replace the canonical mgc.lock file.
//! Ghi staging bền vững và thay file mgc.lock canonical atomic.

use mgc_types::{MgError, MgResult};
use std::path::Path;

use crate::install::cargo_lock::replace_lock_file;

/// Atomically publish the serialized lock so a crash cannot leave a
/// truncated `mgc.lock`. The CLI mutation gateway serializes project
/// writers; this ensures durable file replacement. (Ghi lock nguyên tử.)
pub(in crate::install) fn atomic_write_canonical_lock(path: &Path, bytes: &[u8]) -> MgResult<()> {
    mgc_lockfile::ensure_lockfile_mutation_allowed(path)
        .map_err(|error| MgError::Other(error.to_string()))?;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| MgError::Other("mgc.lock path has no valid filename".to_string()))?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| MgError::Other(format!("system clock is invalid: {e}")))?
        .as_nanos();
    let temp = parent.join(format!(".{filename}.{}.{nonce:x}.tmp", std::process::id()));
    let result = (|| {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW);
        }
        let mut file = options.open(&temp).map_err(|e| {
            MgError::Other(format!(
                "cannot create lock staging file '{}': {e}",
                temp.display()
            ))
        })?;
        file.write_all(bytes)
            .map_err(|e| MgError::Other(format!("cannot write lock staging file: {e}")))?;
        file.sync_all()
            .map_err(|e| MgError::Other(format!("cannot sync lock staging file: {e}")))?;
        replace_lock_file(&temp, path)
            .map_err(|e| MgError::Other(format!("cannot atomically replace mgc.lock: {e}")))?;
        #[cfg(unix)]
        std::fs::File::open(parent)
            .and_then(|dir| dir.sync_all())
            .map_err(|e| MgError::Other(format!("cannot sync lock directory: {e}")))?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result
}

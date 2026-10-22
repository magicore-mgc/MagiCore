//! Enforces signed-lock rewrite policy and atomically publishes verified v4 bytes.
//! Áp policy lock đã ký và publish byte v4 đã xác minh theo cách atomic.

use std::path::{Path, PathBuf};

use crate::LockfileError;

use super::error::V4WriteError;

/// Refuse rewriting a signed v4 lock with an ordinary unsigned writer.
/// Từ chối rewrite lock v4 đã ký bằng writer unsigned thông thường.
///
/// Unsigned v4 passes because its integrity is contained in the single-file transaction.
/// v4 không ký được phép ghi vì integrity nằm trong transaction một file.
pub fn ensure_v4_unsigned_for_rewrite(
    doc: &crate::canonical::LockfileV4,
) -> crate::LockfileResult<()> {
    if doc.metadata.signature.is_some() {
        return Err(V4WriteError::SignedRewrite.into());
    }
    Ok(())
}

/// Atomically replace the lock with verified v4 bytes using create-new staging, fsync,
/// atomic replacement, and directory sync. The caller owns signature policy.
/// Thay lock bằng byte v4 đã xác minh qua staging create-new, fsync, replace atomic
/// và sync directory. Caller chịu trách nhiệm policy chữ ký.
pub fn atomic_write_v4_bytes(dest: &Path, final_bytes: &[u8]) -> crate::LockfileResult<()> {
    write_v4_bytes(dest, final_bytes).map_err(LockfileError::from)
}

fn write_v4_bytes(dest: &Path, final_bytes: &[u8]) -> Result<(), V4WriteError> {
    use std::io::Write;
    let dir: PathBuf = dest
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let name = dest
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or(V4WriteError::InvalidFilename)?;
    let tmp_path = dir.join(format!(
        ".{}.v4.{}.{}.tmp",
        name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    let result = (|| -> Result<(), V4WriteError> {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW);
        }
        #[cfg(windows)]
        {
            if std::fs::symlink_metadata(&tmp_path).is_ok_and(|m| m.file_type().is_symlink()) {
                return Err(V4WriteError::SymlinkTemp);
            }
        }
        let mut tmp = options
            .open(&tmp_path)
            .map_err(|source| V4WriteError::CreateTemp { source })?;
        tmp.write_all(final_bytes)
            .map_err(|source| V4WriteError::WriteTemp { source })?;
        tmp.sync_all()
            .map_err(|source| V4WriteError::SyncTemp { source })?;
        drop(tmp);
        crate::atomic::atomic_replace_file(&tmp_path, dest)
            .map_err(|source| V4WriteError::Replace { source })?;
        #[cfg(unix)]
        std::fs::File::open(&dir)
            .and_then(|directory| directory.sync_all())
            .map_err(|source| V4WriteError::SyncDirectory { source })?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp_path);
    }
    result
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

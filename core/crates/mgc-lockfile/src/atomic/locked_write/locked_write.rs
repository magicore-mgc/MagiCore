//! Writes lock bytes through a unique, durable temp while holding project ownership.
//! Ghi byte lock qua temp duy nhất, bền vững khi đang giữ quyền sở hữu project.

use std::path::{Path, PathBuf};
use std::time::Duration;

use super::super::stale_temp::temp_name;
use crate::atomic::{atomic_replace_file, cleanup_stale_temps, lock_failpoint};
use crate::{LockfileError, LockfileResult};

use super::error::LockedWriteError;

/// Write the complete v4 document (including hash and signature) atomically.
/// Ghi atomic document v4 hoàn chỉnh, bao gồm hash và signature.
///
/// Requires the project writer lock; the type system enforces this ownership.
/// Bắt buộc project writer lock; kiểu dữ liệu cưỡng chế quyền sở hữu này.
pub fn atomic_write_locked(
    _lock: &crate::project_lock::ProjectWriteLock,
    dest: &Path,
    final_bytes: &[u8],
    tmp_grace: Duration,
) -> LockfileResult<()> {
    write_locked(_lock, dest, final_bytes, tmp_grace).map_err(LockfileError::from)
}

fn write_locked(
    _lock: &crate::project_lock::ProjectWriteLock,
    dest: &Path,
    final_bytes: &[u8],
    tmp_grace: Duration,
) -> Result<(), LockedWriteError> {
    let dir: PathBuf = dest
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    cleanup_stale_temps(&dir, tmp_grace);

    lock_failpoint("lock-before-temp-write");
    let tmp_path = dir.join(temp_name());
    {
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
                return Err(LockedWriteError::SymlinkTemp);
            }
        }
        let mut tmp = options
            .open(&tmp_path)
            .map_err(|source| LockedWriteError::CreateTemp {
                path: tmp_path.display().to_string(),
                source,
            })?;
        use std::io::Write;
        tmp.write_all(final_bytes)
            .map_err(|source| LockedWriteError::WriteTemp {
                path: tmp_path.display().to_string(),
                source,
            })?;
        tmp.sync_all()
            .map_err(|source| LockedWriteError::SyncTemp {
                path: tmp_path.display().to_string(),
                source,
            })?;
    }

    lock_failpoint("lock-after-temp-fsync");
    lock_failpoint("lock-before-rename");
    atomic_replace_file(&tmp_path, dest).map_err(|source| {
        let _ = std::fs::remove_file(&tmp_path);
        LockedWriteError::Replace {
            path: dest.display().to_string(),
            source,
        }
    })?;
    lock_failpoint("lock-after-rename");

    // Persist the rename itself when the directory can be opened.
    // Bảo đảm rename bền vững khi mở được directory.
    if let Ok(dir_handle) = std::fs::File::open(&dir)
        && let Err(source) = dir_handle.sync_all()
    {
        return Err(LockedWriteError::SyncDirectory {
            path: dir.display().to_string(),
            source,
        });
    }
    lock_failpoint("lock-after-dir-fsync");
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

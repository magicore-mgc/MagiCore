//! Lock snapshot/restore and stale lifecycle-snapshot sweeping.
//! Snapshot/khôi phục lock và dọn snapshot vòng đời cũ.

use mgc_types::{MgError, MgResult};
use std::path::Path;

/// back, `None` (no lock existed) removes the file the failed install
/// wrote. Callers combine the io error with the original failure —
/// never a silent drop, never a fake-safe state.
/// (Khôi phục lock cũ — lỗi gộp với lỗi gốc.)
pub(crate) fn restore_prior_lock_result(
    project_root: &Path,
    prior: &Option<Vec<u8>>,
) -> std::io::Result<()> {
    let path = project_root.join("mgc.lock");
    match prior {
        Some(bytes) => std::fs::write(&path, bytes).map(|_| ()),
        None => std::fs::remove_file(&path).or_else(|e| {
            use std::io::ErrorKind;
            if e.kind() == ErrorKind::NotFound {
                Ok(())
            } else {
                Err(e)
            }
        }),
    }
}

/// Snapshot the lock bytes for lifecycle rollback without turning an I/O
/// error, directory, or symlink into the semantic state "no lock existed".
/// That distinction is security-critical: rollback with `None` removes the
/// lock written by the current install.
/// (Snapshot phân biệt NotFound với lỗi đọc, thư mục và symlink; None sẽ xóa
/// lock mới khi rollback nên không được nuốt lỗi thành None.)
pub(crate) fn snapshot_prior_lock(project_root: &Path) -> MgResult<Option<Vec<u8>>> {
    let path = project_root.join("mgc.lock");
    match mgc_lockfile::read_lockfile_bytes(&path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            Ok(None)
        }
        Err(error) => Err(MgError::Other(format!(
            "cannot safely snapshot pre-install lock '{}': {error}",
            path.display()
        ))),
    }
}

/// Remove stale lifecycle-snapshot dirs (`.mgc-snap-*`) left by
/// SIGKILLed installs. Returns the count removed. ONLY the exact prefix
/// plus a REAL directory is touched: `symlink_metadata` (never following
/// links) gates both the prefix match and the dir check, so a hostile
/// `.mgc-snap-*` symlink can neither cause deletion outside the project
/// nor be "cleaned" into anything. Real trees and files are never
/// candidates. Safe under the project install lock (no concurrent
/// install mutates this dir).
/// (Xóa snapshot cũ do SIGKILL — chỉ đúng prefix + thư mục thật, không
/// follow symlink.)
pub(crate) fn sweep_stale_snapshots(project_root: &Path) -> usize {
    let mut swept = 0usize;
    let Ok(entries) = std::fs::read_dir(project_root) else {
        return 0;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(meta) = std::fs::symlink_metadata(&path) else {
            continue;
        };
        if !meta.file_type().is_dir() {
            continue;
        }
        if !entry
            .file_name()
            .to_string_lossy()
            .starts_with(".mgc-snap-")
        {
            continue;
        }
        if std::fs::remove_dir_all(&path).is_ok() {
            swept += 1;
        }
    }
    swept
}

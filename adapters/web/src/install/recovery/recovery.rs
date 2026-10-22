//! Crash recovery and lock snapshot/restore helpers for Web installs.
//! Helper phục hồi crash và snapshot/restore lock cho Web install.

use mgc_types::{MgError, MgResult};
use std::path::Path;
use tokio::task::JoinSet;

/// Outcome of crash recovery (unit-assertable).
/// (Kết quả phục hồi crash — assert được.)
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CrashRecovery {
    pub restored_backup: bool,
    pub removed_staging: usize,
    pub removed_snaps: usize,
}

/// Wait for every lifecycle worker before allowing the caller to roll back
/// the install tree. Returning on the first error would drop the JoinSet while
/// sibling workers may still be running synchronous child processes that can
/// mutate the tree after rollback.
/// (Chờ hết mọi lifecycle worker trước rollback; trả sớm có thể để tiến trình
/// con còn chạy và ghi vào cây đã phục hồi.)
pub(crate) async fn drain_lifecycle_tasks(tasks: &mut JoinSet<MgResult<()>>) -> Option<String> {
    let mut first_error = None;
    while let Some(result) = tasks.join_next().await {
        let error = match result {
            Ok(Ok(())) => continue,
            Ok(Err(error)) => format!("lifecycle script failed — install aborted: {error}"),
            Err(error) => format!("lifecycle script task panicked — install aborted: {error}"),
        };
        if first_error.is_none() {
            first_error = Some(error);
        }
    }
    first_error
}

/// Recover from SIGKILLed installs: restore one unambiguous backup and
/// remove stale staging/snapshot litter under the project install lock.
/// Multiple backups or a failed restore abort recovery without discarding
/// candidate data.
/// (Phục hồi sau SIGKILL: chỉ dựng khi có một backup rõ ràng; backup mơ hồ
/// hoặc restore lỗi sẽ dừng mà không xóa dữ liệu dự phòng.)
pub(crate) fn recover_interrupted_install(
    project_root: &Path,
    node_modules: &Path,
    staging_tmp: &Path,
) -> MgResult<CrashRecovery> {
    // Discover recovery candidates before deleting staging or snapshots.
    // More than one backup means the on-disk history is ambiguous; choosing
    // by mtime and deleting the rest can destroy the only recoverable tree.
    // Preserve every candidate and stop the install for explicit recovery.
    // (Phát hiện backup trước khi dọn rác. Nhiều backup là lịch sử mơ hồ;
    // không chọn theo mtime rồi xóa bản còn lại.)
    let mut backups = Vec::new();
    let entries = std::fs::read_dir(project_root).map_err(|e| {
        MgError::Other(format!(
            "cannot inspect interrupted-install backups in '{}': {e}",
            project_root.display()
        ))
    })?;
    for entry in entries {
        let entry = entry.map_err(|e| {
            MgError::Other(format!(
                "cannot enumerate interrupted-install backups in '{}': {e}",
                project_root.display()
            ))
        })?;
        if !entry
            .file_name()
            .to_string_lossy()
            .starts_with(".mgc-prev-")
        {
            continue;
        }
        let path = entry.path();
        let meta = std::fs::symlink_metadata(&path).map_err(|e| {
            MgError::Other(format!(
                "cannot inspect interrupted-install backup '{}': {e}",
                path.display()
            ))
        })?;
        // Never follow or remove an attacker-controlled symlink as a backup.
        if meta.file_type().is_symlink() {
            continue;
        }
        if meta.file_type().is_dir() {
            backups.push(path);
        }
    }
    if backups.len() > 1 {
        return Err(MgError::Other(format!(
            "found {} interrupted-install backups in '{}'; refusing to choose by timestamp and preserving all candidates. Inspect the .mgc-prev-* directories and recover node_modules manually before retrying",
            backups.len(),
            project_root.display()
        )));
    }

    let mut out = CrashRecovery {
        restored_backup: false,
        removed_staging: 0,
        removed_snaps: sweep_stale_snapshots(project_root),
    };
    // Stale staging dirs (legacy + new) are never live: delete outright.
    // (Staging cũ không bao giờ live: xóa thẳng.)
    for dir in [project_root, staging_tmp] {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(meta) = std::fs::symlink_metadata(&path) else {
                continue;
            };
            if !meta.file_type().is_dir() {
                continue;
            }
            let name = entry.file_name();
            let s = name.to_string_lossy();
            if (s.starts_with(".mgc-stage-") || s.starts_with("install-stage-"))
                && std::fs::remove_dir_all(&path).is_ok()
            {
                out.removed_staging += 1;
            }
        }
    }
    if let Some(backup) = backups.pop() {
        if node_modules.exists() {
            std::fs::remove_dir_all(node_modules).map_err(|e| {
                MgError::Other(format!(
                    "cannot clear interrupted install path '{}' while preserving backup '{}': {e}",
                    node_modules.display(),
                    backup.display()
                ))
            })?;
        }
        std::fs::rename(&backup, node_modules).map_err(|e| {
            MgError::Other(format!(
                "cannot restore interrupted-install backup '{}' to '{}': {e}; backup preserved",
                backup.display(),
                node_modules.display()
            ))
        })?;
        out.restored_backup = true;
    }
    Ok(out)
}

/// Restore the pre-install lockfile (P0-1): `Some(bytes)` writes them
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

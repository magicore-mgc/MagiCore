//! Guarded replacement and rollback for the live Web dependency tree.
//! Bảo vệ, thay thế và rollback cây dependency Web.

use super::error::TreeBackupError;
use mgc_types::{MgError, MgResult};
use std::fs::Metadata;
use std::path::{Path, PathBuf};

/// Pre-install tree backup with crash recovery (P0-2): the live
/// `node_modules` is renamed aside (ONE atomic rename, same filesystem)
/// before any mutation, and the fresh tree materializes in its place.
/// Success deletes the backup (`commit`); ANY failure path — `?` early
/// returns, script errors, even panics — hits `Drop`, which renames the
/// backup home. A SIGKILLed run leaves the backup behind; the next
/// install's `recover_interrupted_install` restores it before doing
/// anything else. This is the whole "transaction": one atomic rename
/// per direction, no partial-tree states.
/// (Backup cây pre-install + phục hồi crash: một rename atomic mỗi
/// chiều, không trạng thái cây dở dang.)
pub(crate) struct TreeBackup {
    live: PathBuf,
    backup: Option<PathBuf>,
}

impl TreeBackup {
    /// No-op backup for skip/no-change installs (nothing will be
    /// rewritten — commit/Drop are harmless no-ops).
    /// (Backup rỗng cho install không đổi gì.)
    pub(crate) fn none(live: &Path) -> Self {
        Self {
            live: live.to_path_buf(),
            backup: None,
        }
    }

    fn backup_path(live: &Path) -> PathBuf {
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        live.parent()
            .unwrap_or_else(|| Path::new("."))
            .join(format!(".mgc-prev-{}-{}-{}", std::process::id(), nanos, n))
    }

    fn inspect_live_tree(live: &Path) -> MgResult<Option<Metadata>> {
        let metadata = match std::fs::symlink_metadata(live) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(source) => {
                return Err(MgError::Other(
                    TreeBackupError::InspectLiveTree {
                        path: live.to_path_buf(),
                        source,
                    }
                    .to_string(),
                ));
            }
        };
        if metadata.file_type().is_symlink() {
            return Err(MgError::Other(
                TreeBackupError::LiveTreeIsSymlink {
                    path: live.to_path_buf(),
                }
                .to_string(),
            ));
        }
        if !metadata.is_dir() {
            return Err(MgError::Other(
                TreeBackupError::LiveTreeNotDirectory {
                    path: live.to_path_buf(),
                }
                .to_string(),
            ));
        }
        Ok(Some(metadata))
    }

    /// Validate the project install-tree path before setup creates stores or directories.
    /// Kiểm tra đường dẫn cây install trước khi setup tạo store hoặc thư mục.
    pub(crate) fn preflight_target(live: &Path) -> MgResult<()> {
        Self::inspect_live_tree(live).map(|_| ())
    }

    /// Move the live tree aside (atomic rename) and recreate an empty
    /// live dir. Absent live tree = nothing to protect. A live path the
    /// owner marked READ-ONLY is refused outright (fail-closed): silently
    /// replacing an intentionally immutable tree would violate the
    /// admin's intent — surface it with the path instead.
    /// (Dời cây live sang bên. Cây read-only thì từ chối rõ ràng.)
    pub(crate) fn take(live: &Path) -> MgResult<Self> {
        // Empty dir carries no state (the fn-top create_dir_all always
        // ensures the path exists) — backing it up would later restore
        // emptiness OVER a freshly materialized tree. Skip it.
        // (Dir trống không có state — backup nó sẽ xóa cây mới materialize.)
        let Some(metadata) = Self::inspect_live_tree(live)? else {
            return Ok(Self::none(live));
        };
        let mut entries = std::fs::read_dir(live).map_err(|source| {
            MgError::Other(
                TreeBackupError::InspectLiveTree {
                    path: live.to_path_buf(),
                    source,
                }
                .to_string(),
            )
        })?;
        let is_empty = match entries.next() {
            None => true,
            Some(Ok(_)) => false,
            Some(Err(source)) => {
                return Err(MgError::Other(
                    TreeBackupError::ReadLiveEntry {
                        path: live.to_path_buf(),
                        source,
                    }
                    .to_string(),
                ));
            }
        };
        if is_empty {
            return Ok(Self {
                live: live.to_path_buf(),
                backup: None,
            });
        }
        if metadata.permissions().readonly() {
            return Err(MgError::Other(format!(
                "cannot stage install: existing '{}' is read-only — refusing to replace an immutable tree (adjust permissions to reinstall)",
                live.display()
            )));
        }
        let backup = Self::backup_path(live);
        std::fs::rename(live, &backup).map_err(|e| {
            MgError::Other(format!(
                "cannot stage pre-install backup '{}' -> '{}': {e} (install aborted before mutating the tree)",
                live.display(),
                backup.display()
            ))
        })?;
        std::fs::create_dir_all(live).map_err(|e| {
            MgError::Other(format!(
                "cannot recreate '{}' after backup: {e}",
                live.display()
            ))
        })?;
        Ok(Self {
            live: live.to_path_buf(),
            backup: Some(backup),
        })
    }

    /// Restore the backup NOW, returning every failure as one string.
    /// On success the guard disarms itself (Drop becomes a no-op).
    /// (Khôi phục backup ngay, lỗi gộp thành chuỗi.)
    pub(crate) fn rollback(&mut self) -> Result<(), String> {
        let Some(backup) = self.backup.take() else {
            return Ok(());
        };
        let mut failures: Vec<String> = Vec::new();
        if self.live.exists()
            && let Err(e) = std::fs::remove_dir_all(&self.live)
        {
            failures.push(format!(
                "rollback clear '{}' failed: {e}",
                self.live.display()
            ));
        }
        if failures.is_empty()
            && let Err(e) = std::fs::rename(&backup, &self.live)
        {
            failures.push(format!(
                "rollback rename '{}' -> '{}' failed: {e}",
                backup.display(),
                self.live.display()
            ));
        }
        if failures.is_empty() {
            Ok(())
        } else {
            // Leave backup in place for Drop to retry loudly.
            // (Giữ backup để Drop thử lại ồn ào.)
            self.backup = Some(backup);
            Err(failures.join("; "))
        }
    }

    /// Success path: delete the backup (litter on failure is swept by
    /// the next run's recovery).
    /// (Thành công: xóa backup.)
    pub(crate) fn commit(mut self) {
        if let Some(backup) = self.backup.take()
            && let Err(e) = std::fs::remove_dir_all(&backup)
        {
            eprintln!(
                "[magicore] backup cleanup '{}' failed: {e} (swept on next install)",
                backup.display()
            );
        }
    }
}

impl Drop for TreeBackup {
    fn drop(&mut self) {
        // Every `?` early return, every script arm, every panic lands
        // here when uncommitted: best-effort restore, always loud, never
        // masking (callers add their own error on top).
        // (Mọi đường lỗi chưa commit đều qua đây: cố khôi phục, luôn ồn.)
        if self.backup.is_some()
            && let Err(e) = self.rollback()
        {
            eprintln!("[magicore] backup auto-restore failed: {e}");
        }
    }
}

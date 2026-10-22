//! Export File owner.
//! Phụ trách export file trong luồng export CAS.

use std::fs;
use std::path::Path;

use crate::cas::integrity::IntegrityHash;
use crate::cas::security::check_symlink_ancestors;
use crate::cas::store::{ContentStore, StoreError};
use crate::cas::write::{sync_parent_dir, unique_tmp_path};

use super::super::export_permissions::set_exported_permissions;
use super::super::stage_export::{StageExportError, stage_export_bytes};
use super::super::staged_hash::hash_file;
use super::error::open_cas_blob_source;

impl ContentStore {
    /// Export a CAS blob into a project directory as an INDEPENDENT copy.
    /// Hardlinks were removed: a project-side mutation must never write
    /// through into the shared store (P0 shared-CAS poisoning).
    ///
    /// P0-2 (anti-EXDEV): the staging temp is created INSIDE THE DESTINATION
    /// directory (`.mgc-export-<unique>` next to the final path), so the
    /// final rename is always same-filesystem and never fails with EXDEV
    /// when the store and the project live on different volumes (Docker
    /// mounts, external drives, CI mounts, network shares).
    ///
    /// COW strategy (P0-4): macOS `fclonefileat` and Linux FICLONE are
    /// attempted first using the already-open CAS source handle. Other
    /// platforms/filesystems use a regular copy from that same handle.
    /// Copy-on-write keeps the export independent; the staged result is
    /// hashed before publication and a mismatch is rejected.
    ///
    /// Xuất blob CAS ra project bằng BẢN SAO/CLONE ĐỘC LẬP. Bỏ hardlink:
    /// project sửa file không thể ghi ngược vào shared store (chống đầu
    /// độc P0). P0-2: temp dựng NGAY TRONG thư mục đích (`.mgc-export-*`)
    /// nên rename cuối luôn cùng filesystem — không EXDEV khi store và
    /// project khác volume. P0-4: macOS `fclonefileat` và Linux FICLONE
    /// dùng cùng handle CAS đã mở; platform/filesystem khác copy thường từ
    /// handle đó. File stage độc lập và được hash trước khi publish.
    pub fn export_to(&self, hash: &IntegrityHash, dest: &Path) -> Result<(), StoreError> {
        self.export_to_mode(hash, dest, true, true)
    }

    pub(in crate::cas::store::export) fn export_to_mode(
        &self,
        hash: &IntegrityHash,
        dest: &Path,
        sync_parent_directory: bool,
        verify_source_first: bool,
    ) -> Result<(), StoreError> {
        let parent = dest.parent().ok_or_else(|| StoreError::Io {
            path: dest.to_path_buf(),
            msg: "export destination has no parent directory".to_string(),
        })?;
        fs::create_dir_all(parent)?;
        // Check the destination parent for symlinks (dest doesn't exist yet).
        // Kiểm tra thư mục cha của đích có symlink không (dest chưa tồn tại).
        check_symlink_ancestors(parent)?;

        let src = hash.cas_path(&self.root);
        let source =
            open_cas_blob_source(&src, hash).map_err(|error| error.into_store_error(&src))?;

        if dest.exists() {
            let dest_hash = self.verify(dest)?;
            if dest_hash.as_hex() == hash.as_hex() {
                // Vòng-3 audit P1-3: identical bytes are not enough — the
                // executable SEMANTIC must also match, otherwise an export of
                // an executable blob onto a stale 0644 file reports success
                // while producing a wrong-mode artifact. Repair the mode on
                // the existing destination (idempotent permission fix).
                // (P1-3 audit vòng-3: bytes giống nhau chưa đủ — semantic
                // executable cũng phải khớp, nếu không export blob exec
                // lên file 0644 cũ sẽ báo thành công nhưng ra artifact sai
                // mode. Sửa mode trên đích đã tồn tại (fix permission
                // idempotent).)
                set_exported_permissions(dest, hash.is_executable())?;
                return Ok(());
            }
            return Err(StoreError::Io {
                path: dest.to_path_buf(),
                msg: "destination already exists".to_string(),
            });
        }

        let source = if verify_source_first {
            self.verify_opened_blob(hash, &src, source)?
        } else {
            source
        };

        // P0-2: stage INSIDE the destination filesystem — never the CAS tmp
        // dir — so the final rename cannot hit EXDEV on cross-volume setups.
        // P0-2: staging TRONG filesystem đích — không dùng tmp của CAS —
        // rename cuối không thể gặp EXDEV khi khác volume.
        let tmp = unique_tmp_path(parent, "mgc-export");
        let export_guard = ExportTempGuard { path: &tmp };

        // COW first, plain copy fallback — both produce an independent file.
        // COW trước, copy thường dự phòng — cả hai đều ra file độc lập.
        stage_export_bytes(&source, &tmp).map_err(StageExportError::into_store_error)?;

        // P0-3: the staged copy is the fail-closed integrity gate before publish.
        // Hashing it validates the exact bytes consumers will receive and avoids
        // a redundant source read; a mismatch re-verifies the same open handle.
        //
        // (P0-3: bản staged là cổng integrity fail-closed trước publish.
        // Hash bản này xác minh đúng bytes người dùng sẽ nhận và bỏ một lượt
        // đọc nguồn dư; nếu lệch, xác minh lại đúng handle nguồn đang mở.)
        let staged_hash = hash_file(&tmp)?;
        if staged_hash != hash.as_hex() {
            drop(export_guard);
            let _ = fs::remove_file(&tmp);
            self.verify_opened_blob(hash, &src, source)?;
            return Err(StoreError::HashMismatch {
                expected: hash.as_hex().to_string(),
                actual: staged_hash,
            });
        }

        // Same-filesystem atomic publish (P0-2) + crash-durable parent
        // fsync (audit vòng-3 P1-7).
        // Publish nguyên tử cùng filesystem (P0-2) + fsync cha bền vững
        // khi crash (P1-7 audit vòng-3).
        if let Err(e) = fs::rename(&tmp, dest) {
            return Err(StoreError::Io {
                path: dest.to_path_buf(),
                msg: format!("move exported copy into place failed: {e}"),
            });
        }
        drop(export_guard);
        set_exported_permissions(dest, hash.is_executable())?;
        if sync_parent_directory {
            sync_parent_dir(parent)?;
        }

        Ok(())
    }
}

/// RAII cleanup for the export staging temp — removes the temp file on every
/// early-return path (P0-2/P0-3 discipline: never leak staging files).
/// Dọn staging temp bằng RAII — xóa file temp ở mọi đường return sớm
/// (kỷ luật P0-2/P0-3: không rò rỉ file staging).
struct ExportTempGuard<'a> {
    path: &'a Path,
}

impl Drop for ExportTempGuard<'_> {
    fn drop(&mut self) {
        // Best-effort only: after a successful rename the temp no longer
        // exists and removal is a harmless no-op.
        // Chỉ best-effort: sau rename thành công temp không còn tồn tại,
        // remove là no-op vô hại.
        let _ = fs::remove_file(self.path);
    }
}

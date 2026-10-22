//! Compiled-cache publication race and recovery methods.
//! Method race publish và phục hồi compiled-cache.

use super::super::{CompilationKey, CompiledCache, CompiledRecord, VerifyOutcome};
use crate::cas::store::StoreError;
use crate::cas::write::sync_parent_dir;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

impl CompiledCache {
    /// Decide the outcome after losing the publication race (P0-2).
    /// The winner is read, verified, and compared — never trusted blindly.
    ///
    /// Quyết định kết quả sau khi thua race publish (P0-2). Winner được
    /// đọc, verify, và so sánh — không bao giờ tin mù.
    pub(in crate::cas::compiled_cache) fn resolve_race(
        &self,
        key: &CompilationKey,
        path: &Path,
        tmp: &Path,
        ours: &CompiledRecord,
    ) -> Result<(), StoreError> {
        let winner = match self.read_record(path) {
            Ok(w) => w,
            // Torn/unparseable winner bytes — quarantine whatever is there
            // and take over the slot with our fresh record (self-healing).
            // (Bytes winner đứt/không parse được — cách ly phần còn lại và
            // nhận slot bằng record mới (tự sửa).)
            Err(e) => {
                self.quarantine_corrupt(path, key, &format!("{e}"))?;
                return self.publish_retry(path, tmp);
            }
        };

        match winner.verify_against(key) {
            // Winner is valid AND identical → legitimate dedup.
            // (Winner hợp lệ VÀ giống hệt → dedup hợp lệ.)
            VerifyOutcome::Authentic if winner.output_digest() == ours.output_digest() => Ok(()),
            // Winner is valid but DIVERGENT — same key produced two
            // different outputs: nondeterministic compilation or tampering.
            // Never fake success; leave the valid winner in place.
            // (Winner hợp lệ nhưng PHÂN KỲ — cùng key sinh 2 output khác
            // nhau: biên dịch không tất định hoặc bị can thiệp. Không giả
            // thành công; giữ nguyên winner hợp lệ.)
            VerifyOutcome::Authentic => Err(StoreError::CacheConflict {
                path: path.to_path_buf(),
                expected: ours.output_digest().to_string(),
                winner: winner.output_digest().to_string(),
            }),
            // Winner is STALE (old schema) — remove it and publish the
            // fresh record in its place (exactly one retry).
            // (Winner STALE (schema cũ) — xóa nó và publish record mới vào
            // chỗ của nó (retry đúng 1 lần).)
            VerifyOutcome::StaleSchema => {
                fs::remove_file(path)?;
                self.publish_retry(path, tmp)
            }
            // Winner is CORRUPT — quarantine for forensics, then publish
            // the fresh record in its place (self-healing).
            // (Winner HỎNG — cách ly để điều tra, rồi publish record mới
            // vào chỗ của nó (tự sửa).)
            VerifyOutcome::Failed(reason) => {
                self.quarantine_corrupt(path, key, &reason)?;
                self.publish_retry(path, tmp)
            }
        }
    }

    /// Link the (already fsynced) temp onto the now-free entry path exactly
    /// once. A failure here surfaces as a conflict — never a silent Ok.
    ///
    /// Link temp (đã fsync sẵn) vào entry path vừa trống đúng 1 lần. Lỗi ở
    /// đây báo conflict — không bao giờ Ok im lặng.
    pub(in crate::cas::compiled_cache) fn publish_retry(
        &self,
        path: &Path,
        tmp: &Path,
    ) -> Result<(), StoreError> {
        match fs::hard_link(tmp, path) {
            Ok(()) => {
                // Same P1-4 discipline as the first-writer arm: surface a
                // temp-remove failure instead of leaving an orphan.
                // (Cùng kỷ luật P1-4 như nhánh writer đầu: báo lỗi
                // remove-temp thay vì để lại mồ côi.)
                fs::remove_file(tmp).map_err(|e| StoreError::Io {
                    path: tmp.to_path_buf(),
                    msg: format!("replaced the entry but failed to remove its temp: {e}"),
                })?;
                if let Some(parent) = path.parent() {
                    sync_parent_dir(parent)?;
                }
                Ok(())
            }
            // A sibling won the replacement race — surface a conflict.
            // (Sibling thắng race thay thế — báo conflict.)
            Err(_e) if path.exists() => Err(StoreError::CacheConflict {
                path: path.to_path_buf(),
                expected: "fresh record after stale/corrupt winner removal".to_string(),
                winner: "sibling published first".to_string(),
            }),
            Err(e) => Err(StoreError::Io {
                path: path.to_path_buf(),
                msg: format!("compiled-cache replace retry failed: {e}"),
            }),
        }
    }

    /// Read + parse the record at `path` without verification.
    /// (Đọc + parse record tại `path` chưa verify.)
    pub(in crate::cas::compiled_cache) fn read_record(
        &self,
        path: &Path,
    ) -> Result<CompiledRecord, StoreError> {
        let data = fs::read(path)?;
        serde_json::from_slice(&data).map_err(|e| StoreError::CacheCorrupt {
            path: path.to_path_buf(),
            detail: format!("failed to parse winner record: {e}"),
        })
    }

    /// Move a corrupt/poisoned entry into `quarantine/` with a forensic
    /// report (best-effort: a quarantine failure must not mask the
    /// integrity error being reported — serving still fails closed).
    ///
    /// Chuyển entry hỏng/đầu độc vào `quarantine/` kèm báo cáo điều tra
    /// (best-effort: lỗi cách ly không được che lỗi integrity đang báo —
    /// serving vẫn fail cứng).
    pub(in crate::cas::compiled_cache) fn quarantine_corrupt(
        &self,
        path: &Path,
        key: &CompilationKey,
        reason: &str,
    ) -> Result<(), StoreError> {
        let qdir = self.root.join("quarantine");
        let _ = fs::create_dir_all(&qdir);
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let master = key.master_digest();
        let master8 = &master.as_hex()[..8];
        let qfile = qdir.join(format!("compiled-{master8}-{stamp}.json"));
        let report = qdir.join(format!("compiled-{master8}-{stamp}.report"));
        // Best-effort: quarantine failure must not mask the mismatch error.
        // (Best-effort: lỗi quarantine không được che lỗi mismatch gốc.)
        let _ = fs::rename(path, &qfile);
        let _ = fs::write(
            &report,
            format!(
                "compiled-cache integrity failure\nmaster key digest: {master8}\nreason: {reason}\nquarantined at: {stamp}\n"
            ),
        );
        Ok(())
    }
}

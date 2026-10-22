//! Defines the typed blob presence state and its database/filesystem lookup.
//! Định nghĩa trạng thái blob typed và truy vấn database/filesystem tương ứng.

use super::super::prelude::*;
use super::super::*;

/// Typed blob presence (audit vòng-3 P1-6): existence alone cannot be
/// trusted as "cache hit" — a torn file still occupies the final path.
/// Callers branch on the honest state instead of a boolean.
///
/// Trạng thái blob typed (P1-6 audit vòng-3): chỉ tồn tại không thể tin là
/// "cache hit" — file đứt vẫn chiếm path cuối. Caller phân nhánh theo trạng
/// thái trung thực thay vì boolean.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPresence {
    /// No DB row, or the backing file is gone.
    /// (Không có row DB, hoặc file nền đã mất.)
    Missing,
    /// File exists and its LENGTH matches the DB record — length only, NOT
    /// a digest verification (renamed from `PresentVerified` per audit
    /// vòng-4 P1: the old name overclaimed; two different payloads with the
    /// same length satisfied it). The digest is fully verified at serve
    /// time in get_blob.
    /// (File tồn tại và ĐỘ DÀI khớp bản ghi DB — chỉ độ dài, KHÔNG phải
    /// verify digest (đổi tên từ `PresentVerified` theo P1 audit vòng-4:
    /// tên cũ claim quá mức; 2 payload khác nhau cùng độ dài vẫn thỏa).
    /// Digest được verify đầy đủ lúc serve trong get_blob.)
    PresentLengthMatched,
    /// File exists but its length disagrees with the DB — treat as
    /// "needs repair", never as a cache hit.
    /// (File có nhưng độ dài lệch DB — coi là "cần sửa", không phải cache
    /// hit.)
    PresentLengthMismatch,
}

impl RegistryStore {
    /// Existence WITH disk awareness (audit vòng-3 P1-6): unlike the old
    /// boolean (which trusted the DB row + a bare path stat), this returns
    /// a TYPED status so a caller can never mistake "DB says present" for
    /// "present and trustworthy" — the same semantics asymmetry the audit
    /// flagged between blob_exists and the hardened serving path.
    ///
    /// - `Missing`: no DB row, or the backing file is gone.
    /// - `PresentLengthMatched`: file exists AND its length matches the
    ///   DB's recorded size (audit vòng-4 P1 rename — length only, NOT a
    ///   digest check; full digest verification happens at serve time in
    ///   get_blob — existence checks must stay cheap; a digest mismatch is
    ///   reported at serve with quarantine).
    /// - `PresentLengthMismatch`: file exists but its size disagrees with
    ///   the DB — almost certainly torn/corrupt; callers should treat this
    ///   as "needs repair", not as a cache hit.
    ///
    /// Existence có ý thức về đĩa (P1-6 audit vòng-3): khác boolean cũ (tin
    /// row DB + stat path trơ), hàm này trả status TYPED để caller không
    /// bao giờ nhầm "DB nói có" thành "có và đáng tin" — chính lệch semantics
    /// mà audit chỉ ra giữa blob_exists và đường serving đã harden.
    ///
    /// - `Missing`: không có row DB, hoặc file nền đã mất.
    /// - `PresentLengthMatched`: file tồn tại VÀ độ dài khớp size DB ghi
    ///   (đổi tên theo P1 audit vòng-4 — chỉ độ dài, KHÔNG phải check
    ///   digest; verify digest đầy đủ diễn ra lúc serve trong get_blob —
    ///   existence check phải giữ rẻ; lệch digest được báo lúc serve kèm
    ///   quarantine).
    /// - `PresentLengthMismatch`: file có nhưng size không khớp DB — gần
    ///   như chắc chắn đứt/hỏng; caller nên coi là "cần sửa", không phải
    ///   cache hit.
    pub async fn blob_exists(&self, digest: &str) -> Result<BlobPresence> {
        let row = sqlx::query("SELECT path, size FROM blobs WHERE digest = ?")
            .bind(digest)
            .fetch_optional(&self.db)
            .await?;
        let Some(r) = row else {
            return Ok(BlobPresence::Missing);
        };
        let key: String = r.get("path");
        let db_size: i64 = r.get("size");

        match self.backend.exists(&key).await? {
            false => Ok(BlobPresence::Missing),
            true => {
                // Cheap integrity probe: length agreement with the DB record.
                // (Thăm dò integrity rẻ: độ dài khớp với bản ghi DB.)
                let actual_len = self.backend.published_len(&key).await?;
                if actual_len == db_size as u64 {
                    Ok(BlobPresence::PresentLengthMatched)
                } else {
                    Ok(BlobPresence::PresentLengthMismatch)
                }
            }
        }
    }
}

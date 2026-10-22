use super::super::super::{errors::CasGenerationError, types::Database};

impl Database {
    /// Abort a staging generation (P0-A): drop ITS OWN claims only — the
    /// install that owns the token decided to give up; nobody else's
    /// generation is touched (a concurrent install's staging claims stay).
    ///
    /// State gate FIRST (Gate 11-A, P0-3 of vòng-11 audit): the old code
    /// deleted the token's claims BEFORE checking the marker's state, so
    /// abort(promoted_token) erased every claim of the LIVE promoted
    /// generation while its marker stayed — prune then ate blobs the
    /// project was actively using. Now the abort runs in ONE transaction
    /// that (1) verifies the token exists, (2) verifies it is 'staging',
    /// and only then (3) deletes claims + marker together. Promoted →
    /// typed hard error, zero mutation. Missing → idempotent no-op.
    /// (Hủy một staging generation: chỉ xóa claim CỦA CHÍNH nó — install
    /// sở hữu token quyết định bỏ; không đụng generation của ai khác (claim
    /// staging của install song song giữ nguyên). Cổng state TRƯỚC TIÊN
    /// (P0-3): code cũ xóa claim của token TRƯỚC khi check state của
    /// marker, nên abort(promoted_token) xóa sạch claim của generation
    /// promoted ĐANG SỐNG trong khi marker vẫn còn — prune rồi xóa blob
    /// mà project đang dùng. Giờ abort chạy trong MỘT transaction: (1)
    /// verify token tồn tại, (2) verify còn 'staging', rồi MỚI (3) xóa
    /// claim + marker cùng nhau. Promoted → lỗi cứng có type, không
    /// mutation. Mất → no-op idempotent.)
    pub fn abort_cas_generation(
        &self,
        project_root: &str,
        generation: i64,
    ) -> std::result::Result<(), CasGenerationError> {
        // IMMEDIATE transaction — same rationale as promote (see above):
        // verify-then-write in a deferred transaction upgrades locks
        // mid-flight and dies to BUSY_SNAPSHOT under WAL contention.
        // (Transaction IMMEDIATE — cùng lý do như promote (xem trên):
        // verify-rồi-ghi trong deferred transaction nâng lock giữa chừng
        // và chết vì BUSY_SNAPSHOT dưới tranh chấp WAL.)
        self.conn
            .execute_batch("BEGIN IMMEDIATE")
            .map_err(|e| CasGenerationError::Io(e.to_string()))?;
        let result = (|| {
            if !self.validate_cas_generation_abort_state(project_root, generation)? {
                return Ok(());
            }
            self.delete_cas_generation_claims(project_root, generation)?;
            self.retire_cas_generation_marker(project_root, generation)?;
            Ok(())
        })();
        match result {
            Ok(()) => self
                .conn
                .execute_batch("COMMIT")
                .map_err(|e| CasGenerationError::Io(e.to_string())),
            Err(e) => {
                // Best-effort rollback: the primary error already explains
                // the failure; a failed rollback adds nothing and must not
                // mask the original error. (Rollback cố gắng: lỗi gốc đã
                // giải thích thất bại; rollback lỗi không thêm gì và không
                // được che lỗi gốc.)
                let _ = self.conn.execute_batch("ROLLBACK");
                Err(e)
            }
        }
    }
}

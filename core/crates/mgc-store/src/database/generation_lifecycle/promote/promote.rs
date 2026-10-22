use super::super::super::{errors::CasGenerationError, types::Database};
impl Database {
    /// Atomically promote the TOKEN's staging generation (P0-A): in ONE
    /// transaction —
    /// 1. every claim of this token survives;
    /// 2. claims of OTHER generations die ONLY when the token's generation
    ///    still vouches for that hash (dedup) or the other generation is
    ///    NOT a live staging install (an in-flight install's staging claims
    ///    are protected — over-retention, never deletion of a live blob);
    /// 3. the token's generation flips to 'promoted', all OTHER promoted
    ///    generations of this project are retired (their rows are gone per
    ///    rule 2), older still-staging generations are LEFT ALONE for their
    ///    own promote/abort to resolve.
    ///
    /// A crash before this point leaves old ∪ new claims live — always SAFE
    /// to prune against.
    ///
    /// Thăng cấp nguyên tử generation của TOKEN: trong MỘT transaction —
    /// (1) mọi claim của token sống sót; (2) claim của generation KHÁC chỉ
    /// chết khi generation của token vẫn bảo chứng hash đó (dedup) hoặc
    /// generation kia KHÔNG phải install staging đang chạy (claim staging
    /// của install đang chạy được bảo vệ — giữ thừa, không xóa blob sống);
    /// (3) generation của token thành 'promoted', mọi generation 'promoted'
    /// khác của project nghỉ hưu, staging cũ kia giữ nguyên cho
    /// promote/abort của riêng nó giải quyết. Crash trước điểm này để lại
    /// claim cũ ∪ mới — luôn AN TOÀN cho prune.
    pub fn promote_cas_generation(
        &self,
        project_root: &str,
        generation: i64,
    ) -> std::result::Result<(), CasGenerationError> {
        // IMMEDIATE transaction (Gate 11-B hardening): promote reads the
        // token state BEFORE writing — a DEFERRED transaction upgrades
        // read→write mid-flight and hits BUSY_SNAPSHOT (unretryable under
        // WAL contention, observed live by the barrier race test). Taking
        // the write lock up front makes the whole verify+retire+flip one
        // serialized unit; SQLITE_BUSY retries via busy_timeout.
        // (Transaction IMMEDIATE: promote đọc state token TRƯỚC khi ghi —
        // transaction DEFERRED nâng read→write giữa chừng và dính
        // BUSY_SNAPSHOT (không retry được dưới tranh chấp WAL, test barrier
        // bắt được lúc chạy thật). Lấy write lock ngay từ đầu biến toàn bộ
        // verify+retire+flip thành một đơn vị xếp tuần tự; SQLITE_BUSY
        // retry qua busy_timeout.)
        self.conn
            .execute_batch("BEGIN IMMEDIATE")
            .map_err(|e| CasGenerationError::Io(e.to_string()))?;
        let result = self.promote_cas_generation_impl(project_root, generation);
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

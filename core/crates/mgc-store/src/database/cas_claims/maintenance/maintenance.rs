//! CAS live-reference queries and test-only destructive helpers.
//! Truy vấn reference CAS đang sống và helper xóa chỉ dành cho test.
use super::super::super::types::Database;
use anyhow::Result;
#[cfg(feature = "test-util")]
use rusqlite::params;

impl Database {
    /// TEST-ONLY destructive clear (Gate 11-A tightening, vòng-11
    /// verdict): NO production caller exists (verified by call-graph scan
    /// — only refcount.rs tests use it). Under the generation-token
    /// protocol this deletes the claims of EVERY generation of a project
    /// — staging installs included — exactly the live-blob loss the token
    /// gates exist to prevent. It is now feature-gated so production
    /// builds cannot link it; the protocol's only retire paths are
    /// promote/abort (verified) and the doctor's stale-staging GC.
    /// (Clear phá hủy CHỈ-CHO-TEST: không còn caller production nào
    /// (quét call-graph xác minh — chỉ test refcount.rs dùng). Dưới giao
    /// thức token, hàm này xóa claim của MỌI generation của project — cả
    /// install staging — đúng chỗ mất blob sống mà các cổng token tồn tại
    /// để chặn. Giờ khóa theo feature để build production không link
    /// được; đường nghỉ hưu duy nhất của giao thức là promote/abort (đã
    /// verify) và GC stale-staging của doctor.)
    #[cfg(feature = "test-util")]
    pub fn clear_all_cas_refs(&self, project_root: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM cas_blob_refs WHERE project_root = ?1",
            params![project_root],
        )?;
        Ok(())
    }

    /// TEST-ONLY single-claim removal (Gate 11-A tightening): same
    /// rationale as `clear_all_cas_refs` — it deletes the claim row of
    /// EVERY generation holding that hash (staging installs included),
    /// bypassing the token gates. Production code must retire claims via
    /// promote (graph changes) or abort (install failure).
    /// (Xóa 1 claim CHỈ-CHO-TEST: cùng lý do như `clear_all_cas_refs` —
    /// xóa row claim của MỌI generation đang giữ hash đó (kể cả staging),
    /// vượt qua cổng token. Code production phải nghỉ hưu claim qua
    /// promote (graph đổi) hoặc abort (install fail).)
    #[cfg(feature = "test-util")]
    pub fn cas_release(&self, project_root: &str, hash: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM cas_blob_refs WHERE project_root = ?1 AND hash = ?2",
            params![project_root, hash],
        )?;
        Ok(())
    }

    /// Blob hashes with at least one live project claim — must NOT be pruned.
    /// Live = ANY generation (staging or promoted): an in-flight install's
    /// claims protect its blobs, and the previous generation protects the
    /// blobs the new install will reuse (P0-C).
    /// (Hash blob còn ít nhất 1 project claim — không được xóa. Live =
    ///  BẤT KỲ generation nào (staging hay đã promote): claim của install
    ///  đang chạy bảo vệ blob của nó, generation trước bảo vệ blob mà
    ///  install mới sẽ tái dùng (P0-C).)
    pub fn list_cas_live_refs(&self) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT hash FROM cas_blob_refs ORDER BY hash")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}

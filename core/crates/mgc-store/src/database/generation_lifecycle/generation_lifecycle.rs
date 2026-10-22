use super::super::types::Database;
use anyhow::Result;
use rusqlite::params;

impl Database {
    /// Begin a NEW install generation for `project_root` and return its
    /// TOKEN (P0-A): the token is the ONLY identity of this install — claims
    /// must carry it, promote must receive it, and no MAX(generation) global
    /// peek exists anymore (the v1 MAX binding let a concurrent install
    /// hijack another install's claims). The new generation is 'staging'
    /// until promote; the previous generations stay visible to prune.
    /// (Bắt đầu generation install MỚI và trả TOKEN của nó: token là danh
    /// tính DUY NHẤT của install — claim phải mang nó, promote phải nhận
    /// đúng nó, không còn globally peek MAX(generation) (gán MAX v1 cho
    /// install song song cướp claim của install khác). Generation mới là
    /// 'staging' tới khi promote; generation trước vẫn hiện diện với prune.)
    pub fn begin_cas_generation(&self, project_root: &str) -> Result<i64> {
        // Atomic allocation (Gate 11-A, P0-1 of vòng-11 audit — FIXED after
        // the barrier race test PROVED BOTH naive versions raced):
        // (a) `SELECT MAX + 1` as two statements raced outright;
        // (b) `INSERT ... SELECT COALESCE(MAX+1)` still computed MAX from
        //     a pre-lock WAL snapshot;
        // (c) even a serialized MAX read REUSES numbers after an abort
        //     deletes the highest marker — the barrier test caught token
        // 26 handed to two threads. The fix is the AUTOINCREMENT
        // pattern: a `cas_generation_seq` counter that only GROWS
        // (abort never touches it), updated in the same BEGIN IMMEDIATE
        // transaction that inserts the marker. Write lock first, MAX
        // never consulted, tokens never reused — atomic by construction.
        // SQLITE_BUSY is retried by busy_timeout (set FIRST at open).
        // (Cấp phát nguyên tử (P0-1 — sửa sau khi test barrier race CHỨNG
        // MINH cả 3 bản ngây thơ đều đua): (a) `SELECT MAX + 1` tách 2
        // statement đua thẳng; (b) `INSERT ... SELECT COALESCE(MAX+1)` vẫn
        // tính MAX từ snapshot WAL trước lock; (c) kể cả đọc MAX tuần tự
        // vẫn TÁI SỬ DỤNG số sau khi abort xóa marker cao nhất — test bắt
        // được token 26 cấp cho 2 thread. Bản sửa là pattern
        // AUTOINCREMENT: bộ đếm `cas_generation_seq` chỉ TĂNG (abort
        // không bao giờ đụng), cập nhật trong cùng transaction BEGIN
        // IMMEDIATE chèn marker. Write lock trước, MAX không bao giờ bị
        // hỏi, token không bao giờ tái sử dụng — nguyên tử theo cấu
        // trúc. SQLITE_BUSY được retry bởi busy_timeout.)
        self.conn.execute_batch("BEGIN IMMEDIATE")?;
        let result = (|| {
            // Upsert the sequence, RETURNING the issued number — one
            // statement under the write lock.
            // (Upsert sequence, TRẢ VỀ số đã cấp — một statement dưới
            // write lock.)
            let next: i64 = self.conn.query_row(
                "INSERT INTO cas_generation_seq (project_root, last_issued)
                     VALUES (?1, 1)
                 ON CONFLICT(project_root) DO UPDATE
                     SET last_issued = last_issued + 1
                 RETURNING last_issued",
                params![project_root],
                |row| row.get(0),
            )?;
            // Lease stamp (Gate 11-B): the staging marker records WHO
            // began it and WHEN — the doctor's stale-staging GC uses
            // pid-liveness + age to tell a live install from a dead one.
            // (Đóng dấu lease: marker staging ghi AI bắt đầu và KHI NÀO
            // — GC stale-staging của doctor dùng pid-còn-sống + tuổi để
            // phân biệt install đang chạy với install chết.)
            let now_secs = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            self.conn.execute(
                "INSERT INTO cas_generations
                     (project_root, generation, state, lease_pid, lease_started_at)
                 VALUES (?1, ?2, 'staging', ?3, ?4)",
                params![project_root, next, std::process::id() as i64, now_secs],
            )?;
            Ok(next)
        })();
        match result {
            Ok(next) => {
                self.conn.execute_batch("COMMIT")?;
                Ok(next)
            }
            Err(e) => {
                // Roll back best-effort; the error already explains the
                // failure. English-only console (RULE §7).
                // (Rollback cố gắng; lỗi đã giải thích thất bại. Console
                // tiếng Anh (RULE §7).)
                let _ = self.conn.execute_batch("ROLLBACK");
                Err(e)
            }
        }
    }
}

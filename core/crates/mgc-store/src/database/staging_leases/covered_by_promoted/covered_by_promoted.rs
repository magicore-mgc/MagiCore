//! Select staging leases whose claims are fully covered by promoted generations.
//! Chọn staging lease có toàn bộ claim được promoted generation bao phủ.
use super::super::super::types::{Database, StagingLease};
use anyhow::Result;

impl Database {
    /// Claim-map GC query (P2-2, fresh-context review 2026-09-15): the
    /// doctor's old lease GC NEVER retired a claim-FUL staging
    /// generation — a crash between the first claim and promote leaked
    /// its claims FOREVER (safe over-retention, but unbounded). This
    /// query names the RETIRABLE subset with full evidence:
    /// a claim-ful staging generation is garbage iff
    ///   (a) its lease holder is DEAD or the lease is ancient (the
    ///       caller checks pid/age — the row carries both), AND
    ///   (b) EVERY hash it claims is still claimed by a PROMOTED
    ///       generation of the same project — i.e. retiring it deletes
    ///       not a single live-blob protection (the newest promoted
    ///       refset subsumes it).
    /// Rows returned: (project, generation, pid, started_at, claims).
    /// The caller still gates on pid-liveness + grace and performs the
    /// retire through `abort_cas_generation` (token gates stay intact).
    /// (Truy vấn GC claim-map (P2-2): GC lease cũ của doctor KHÔNG BAO
    /// GIỜ nghỉ hưu staging CÓ claim — crash giữa claim đầu và promote
    /// rò claim VĨNH VIỄN (giữ thừa an toàn nhưng không giới hạn). Truy
    /// vấn này gọi tên tập NGHỈ HƯU ĐƯỢC với bằng chứng đầy đủ: staging
    /// có claim là rác iff (a) holder lease CHẾT hoặc lease cổ (caller
    /// check pid/tuổi — row mang cả hai), VÀ (b) MỌI hash nó claim vẫn
    /// được generation PROMOTED của cùng project claim — nghỉ hưu nó
    /// không xóa một bảo vệ blob sống nào (refset promoted mới nhất thâu
    /// hẹp nó). Caller vẫn chặn theo pid/tuổi và nghỉ hưu qua
    /// abort_cas_generation (cổng token giữ nguyên).)
    pub fn list_staging_leases_covered_by_promoted(&self) -> Result<Vec<StagingLease>> {
        let mut stmt = self.conn.prepare(
            "SELECT g.project_root, g.generation, g.lease_pid, g.lease_started_at,
                    (SELECT COUNT(*) FROM cas_blob_refs r
                     WHERE r.project_root = g.project_root
                       AND r.generation = g.generation) AS claims
             FROM cas_generations g
             WHERE g.state = 'staging'
               AND EXISTS (
                   SELECT 1 FROM cas_blob_refs r
                   WHERE r.project_root = g.project_root
                     AND r.generation = g.generation
               )
               AND NOT EXISTS (
                   -- any claimed hash NOT covered by a promoted generation
                   -- of this project → NOT safe to retire
                   -- (hash được claim mà KHÔNG promoted generation nào của
                   -- project này giữ → KHÔNG an toàn để nghỉ hưu)
                   SELECT 1 FROM cas_blob_refs r
                   WHERE r.project_root = g.project_root
                     AND r.generation = g.generation
                     AND NOT EXISTS (
                         SELECT 1 FROM cas_blob_refs p
                         JOIN cas_generations m
                           ON m.project_root = p.project_root
                          AND m.generation = p.generation
                          AND m.state = 'promoted'
                         WHERE p.project_root = r.project_root
                           AND p.hash = r.hash
                     )
               )",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(StagingLease {
                project_root: row.get(0)?,
                generation: row.get(1)?,
                lease_pid: row.get(2)?,
                lease_started_at: row.get(3)?,
                claim_count: row.get(4)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }
}

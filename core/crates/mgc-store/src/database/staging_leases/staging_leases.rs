use super::super::types::{Database, StagingLease};
use anyhow::Result;

impl Database {
    /// Crash-recovery lease query (Gate 11-B): staging generations with
    /// their lease — (project, generation, pid, started_at, claim_count).
    /// A staging generation is GARBAGE only when BOTH the lease holder is
    /// DEAD (pid not alive — checked by the caller, this crate stays
    /// platform-clean) AND the lease is older than the grace window AND
    /// it holds zero claims; anything else may be a LIVE install.
    /// (Truy vấn lease phục hồi crash: các staging generation kèm lease.
    /// Một staging generation là RÁC chỉ khi CẢ holder lease ĐÃ CHẾT (pid
    /// không sống — caller kiểm tra, crate này giữ sạch platform) VÀ lease
    /// cũ hơn cửa sổ xử lý VÀ không giữ claim nào; còn lại có thể là
    /// install đang CHẠY.)
    pub fn list_staging_leases(&self) -> Result<Vec<StagingLease>> {
        let mut stmt = self.conn.prepare(
            "SELECT g.project_root, g.generation, g.lease_pid, g.lease_started_at,
                    (SELECT COUNT(*) FROM cas_blob_refs r
                     WHERE r.project_root = g.project_root
                       AND r.generation = g.generation) AS claims
             FROM cas_generations g
             WHERE g.state = 'staging'",
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

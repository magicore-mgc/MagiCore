//! Read-only CAS integrity counts for store diagnostics.
//! Số liệu toàn vẹn CAS chỉ đọc cho chẩn đoán store.

use super::super::types::Database;
use super::error::CasIntegritySnapshotError;

const CAS_INTEGRITY_COUNTS_SQL: &str = "SELECT
    (SELECT COALESCE(SUM(
        CASE WHEN state = 'promoted' AND (
            SELECT COUNT(*) FROM cas_generations g4
            WHERE g4.project_root = cas_generations.project_root
              AND g4.state = 'promoted'
        ) > 1 THEN 1 ELSE 0 END
    ), 0) FROM cas_generations),
    (SELECT COUNT(*) FROM cas_blob_refs r
     WHERE NOT EXISTS (
        SELECT 1 FROM cas_generations g
        WHERE g.project_root = r.project_root
          AND g.generation = r.generation
     ))";

/// Read-only integrity totals returned to store diagnostics.
/// Tổng số liệu toàn vẹn chỉ đọc trả về cho công cụ chẩn đoán store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CasIntegrityCounts {
    /// Promoted generation rows belonging to projects with multiple promoted generations.
    /// Số row generation promoted thuộc project có nhiều generation promoted.
    pub multi_promoted_generations: i64,
    /// CAS claim rows with no matching generation marker.
    /// Số row CAS claim không có generation marker tương ứng.
    pub orphan_claims: i64,
}

impl Database {
    /// Count promoted-generation conflicts and orphan CAS claims using one read query.
    /// Đếm generation promoted trùng và CAS claim mồ côi bằng một truy vấn đọc.
    pub fn cas_integrity_counts(&self) -> Result<CasIntegrityCounts, CasIntegritySnapshotError> {
        let (multi_promoted_generations, orphan_claims) = self
            .conn
            .query_row(CAS_INTEGRITY_COUNTS_SQL, [], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .map_err(CasIntegritySnapshotError::Query)?;
        Ok(CasIntegrityCounts {
            multi_promoted_generations,
            orphan_claims,
        })
    }
}

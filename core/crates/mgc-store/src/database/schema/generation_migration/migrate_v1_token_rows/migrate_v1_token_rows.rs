//! Migrate generation-bearing v1 claim rows to the token schema.
//! Migrate row claim v1 đã có generation sang schema token.
use anyhow::Result;
use rusqlite::Transaction;

/// Copy v1 generations and references, then replace the legacy tables.
/// (Chép generation/reference v1 rồi thay các bảng cũ.)
pub(in crate::database::database::schema::generation_migration) fn migrate_v1_token_rows(
    tx: &Transaction,
) -> Result<()> {
    // v1 (vòng-7/8 shape): rows keep their generation number;
    // the v1 counter row maps to a 'staging' marker when it is
    // a never-promoted install (>0 — v1 promote resets to 0) —
    // over-retention until abort/doctor, never a lost blob.
    // Gen-0 rows are completed baselines → 'promoted' marker.
    // (v1: row giữ số generation; row counter v1 thành marker
    // 'staging' khi là install chưa promote (>0 — promote v1
    // reset về 0) — giữ thừa tới abort/doctor, không mất blob.
    // Row gen-0 là baseline đã hoàn tất → marker 'promoted'.)
    tx.execute_batch(
        "INSERT OR IGNORE INTO cas_generations_v2 (project_root, generation, state)
                        SELECT project_root,
                               CASE WHEN generation = 0 THEN 0 ELSE generation END,
                               CASE WHEN generation = 0 THEN 'promoted' ELSE 'staging' END
                        FROM cas_generations;
                    INSERT OR IGNORE INTO cas_generations_v2 (project_root, generation, state)
                        SELECT DISTINCT project_root, 0, 'promoted' FROM cas_blob_refs
                        WHERE generation = 0;
                    INSERT OR IGNORE INTO cas_generations_v2 (project_root, generation, state)
                        SELECT DISTINCT project_root, generation, 'staging'
                        FROM cas_blob_refs WHERE generation > 0;
                    INSERT OR IGNORE INTO cas_blob_refs_v2 (project_root, generation, hash)
                        SELECT project_root, generation, hash FROM cas_blob_refs;
                    DROP TABLE cas_blob_refs;
                    DROP TABLE cas_generations;
                    ALTER TABLE cas_blob_refs_v2 RENAME TO cas_blob_refs;
                    ALTER TABLE cas_generations_v2 RENAME TO cas_generations;",
    )?;
    Ok(())
}

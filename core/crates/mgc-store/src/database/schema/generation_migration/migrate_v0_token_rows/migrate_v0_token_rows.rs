//! Migrate pre-generation claims as the promoted generation-zero baseline.
//! Migrate claim trước generation thành baseline promoted generation 0.
use anyhow::Result;
use rusqlite::Transaction;

/// Copy v0 claims into generation zero and replace the legacy tables.
/// (Chép claim v0 vào generation 0 rồi thay các bảng cũ.)
pub(in crate::database::database::schema::generation_migration) fn migrate_v0_token_rows(
    tx: &Transaction,
) -> Result<()> {
    // v0 (pre-generation): every claim becomes the generation-0
    // PROMOTED baseline of its project.
    // (v0: mọi claim thành baseline gen-0 PROMOTED của project.)
    tx.execute_batch(
        "INSERT OR IGNORE INTO cas_generations_v2 (project_root, generation, state)
                        SELECT DISTINCT project_root, 0, 'promoted' FROM cas_blob_refs;
                    INSERT OR IGNORE INTO cas_blob_refs_v2 (project_root, generation, hash)
                        SELECT project_root, 0, hash FROM cas_blob_refs;
                    DROP TABLE cas_blob_refs;
                    DROP TABLE IF EXISTS cas_generations;
                    ALTER TABLE cas_blob_refs_v2 RENAME TO cas_blob_refs;
                    ALTER TABLE cas_generations_v2 RENAME TO cas_generations;",
    )?;
    Ok(())
}

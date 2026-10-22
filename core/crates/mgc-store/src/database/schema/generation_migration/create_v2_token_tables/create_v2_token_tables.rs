//! Create the replacement CAS generation and reference tables.
//! Tạo các bảng thay thế cho CAS generation và reference.
use anyhow::Result;
use rusqlite::Transaction;

/// Create the v2 tables before copying legacy claim data.
/// (Tạo bảng v2 trước khi chép dữ liệu claim cũ.)
pub(in crate::database::database::schema::generation_migration) fn create_v2_token_tables(
    tx: &Transaction,
) -> Result<()> {
    tx.execute_batch(
        "CREATE TABLE IF NOT EXISTS cas_generations_v2 (
                    project_root TEXT NOT NULL,
                    generation INTEGER NOT NULL,
                    state TEXT NOT NULL DEFAULT 'staging'
                        CHECK (state IN ('staging', 'promoted')),
                    PRIMARY KEY (project_root, generation)
                );
                CREATE TABLE IF NOT EXISTS cas_blob_refs_v2 (
                    project_root TEXT NOT NULL,
                    generation INTEGER NOT NULL DEFAULT 0,
                    hash TEXT NOT NULL,
                    PRIMARY KEY (project_root, generation, hash),
                    FOREIGN KEY (project_root, generation)
                        REFERENCES cas_generations_v2 (project_root, generation)
                        ON DELETE CASCADE
                );",
    )?;
    Ok(())
}

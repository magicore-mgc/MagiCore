use anyhow::Result;
use rusqlite::Transaction;

pub(in crate::database::database::schema) fn create_base_tables(tx: &Transaction) -> Result<()> {
    tx.execute_batch(
        "
            CREATE TABLE IF NOT EXISTS packages (
                id TEXT NOT NULL,
                version TEXT NOT NULL,
                integrity TEXT,
                installed_at INTEGER NOT NULL,
                PRIMARY KEY (id, version)
            );
            CREATE TABLE IF NOT EXISTS integrity_cache (
                hash TEXT PRIMARY KEY,
                verified_at INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS refs (
                project_root TEXT NOT NULL,
                package_id TEXT NOT NULL,
                ref_count INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (project_root, package_id)
            );
            CREATE TABLE IF NOT EXISTS cas_blob_refs (
                project_root TEXT NOT NULL,
                generation INTEGER NOT NULL DEFAULT 0,
                hash TEXT NOT NULL,
                PRIMARY KEY (project_root, generation, hash),
                FOREIGN KEY (project_root, generation)
                    REFERENCES cas_generations (project_root, generation)
                    ON DELETE CASCADE
            );
            CREATE INDEX IF NOT EXISTS idx_cas_blob_refs_hash ON cas_blob_refs (hash);
            CREATE TABLE IF NOT EXISTS cas_generations (
                project_root TEXT NOT NULL,
                generation INTEGER NOT NULL,
                state TEXT NOT NULL DEFAULT 'staging'
                    CHECK (state IN ('staging', 'promoted')),
                PRIMARY KEY (project_root, generation)
            );
            CREATE TABLE IF NOT EXISTS package_files (
                id TEXT NOT NULL,
                version TEXT NOT NULL,
                path TEXT NOT NULL,
                blob_hash TEXT NOT NULL,
                size INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (id, version, path)
            );
            CREATE INDEX IF NOT EXISTS idx_package_files_blob ON package_files (blob_hash);
            CREATE TABLE IF NOT EXISTS trust_policy (
                package_id TEXT PRIMARY KEY,
                policy TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS release_policy (
                ecosystem TEXT PRIMARY KEY,
                min_age_secs INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS cas_generation_seq (
                project_root TEXT PRIMARY KEY,
                last_issued INTEGER NOT NULL DEFAULT 0
            );
            ",
    )?;
    Ok(())
}

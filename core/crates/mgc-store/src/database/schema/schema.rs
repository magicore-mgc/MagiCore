use super::super::{claim_writer_gate::shared_claim_write_gate, types::Database};
use super::{
    base_schema::create_base_tables,
    generation_migration::migrate_legacy_token_schema,
    generation_sequence::bootstrap_generation_sequence,
    lease_migration::{ensure_lease_columns, ensure_lease_columns_in_transaction},
    wal_mode::enable_wal_mode,
};
use anyhow::Result;
use rusqlite::{Connection, OpenFlags};
use std::path::Path;

pub(super) const STORE_DB_SCHEMA_VERSION: i64 = 1;
pub(super) const STORE_DB_BUSY_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        open_database_with_flags(path, OpenFlags::default())
    }
}

pub(super) fn open_database_with_flags(path: &Path, flags: OpenFlags) -> Result<Database> {
    let mut conn = Connection::open_with_flags(path, flags)?;
    conn.busy_timeout(STORE_DB_BUSY_TIMEOUT)?;
    let claim_write_gate = shared_claim_write_gate(path)?;
    let _schema_gate = claim_write_gate
        .lock()
        .map_err(|_| anyhow::anyhow!("store claim-writer gate was poisoned"))?;

    let schema_version: i64 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if schema_version > STORE_DB_SCHEMA_VERSION {
        anyhow::bail!(
            "store.db schema version {schema_version} is newer than this MagiCore build supports ({STORE_DB_SCHEMA_VERSION})"
        );
    }
    if schema_version < STORE_DB_SCHEMA_VERSION {
        conn.pragma_update(None, "foreign_keys", false)?;
        // WAL mode is a one-time store initialization step. Avoid asking
        // SQLite to acquire schema/write locks on every package extraction.
        enable_wal_mode(&conn)?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let schema_version: i64 = tx.pragma_query_value(None, "user_version", |row| row.get(0))?;
        if schema_version > STORE_DB_SCHEMA_VERSION {
            anyhow::bail!(
                "store.db schema version {schema_version} is newer than this MagiCore build supports ({STORE_DB_SCHEMA_VERSION})"
            );
        }
        if schema_version >= STORE_DB_SCHEMA_VERSION {
            tx.commit()?;
        } else {
            create_base_tables(&tx)?;
            migrate_legacy_token_schema(&tx)?;
            bootstrap_generation_sequence(&tx)?;
            ensure_lease_columns_in_transaction(&tx)?;

            tx.pragma_update(None, "user_version", STORE_DB_SCHEMA_VERSION)?;
            tx.commit()?;
        }
    }
    ensure_lease_columns(&mut conn)?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;

    // Enable FK enforcement (Gate 11-A, P0-4): MUST run AFTER the
    // legacy migration above — PRAGMA foreign_keys is a no-op inside
    // a transaction, and the migration itself DROPs the parent tables
    // (legal only with FKs off). With this on, cas_blob_refs cannot
    // gain a row whose (project_root, generation) has no marker —
    // the schema-level guarantee behind cas_claim's token gate.
    // (Bật áp dụng FK (P0-4): phải chạy SAU migration legacy phía
    // trên — PRAGMA foreign_keys là no-op trong transaction, và chính
    // migration DROP bảng cha (hợp pháp chỉ khi FK tắt). Bật xong,
    // cas_blob_refs không thể có row mà (project_root, generation)
    // thiếu marker — bảo đảm tầng schema sau cổng token của cas_claim.)
    conn.pragma_update(None, "foreign_keys", true)?;

    drop(_schema_gate);
    Ok(Database {
        conn,
        claim_write_gate,
    })
}

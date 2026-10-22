//! Coordinate migration from legacy CAS generation schemas.
//! Điều phối migrate schema CAS generation cũ.
use super::{
    create_v2_token_tables::create_v2_token_tables,
    generation_token_migration_needed::generation_token_migration_needed,
    legacy_generation_column_present::legacy_generation_column_present,
    migrate_v0_token_rows::migrate_v0_token_rows, migrate_v1_token_rows::migrate_v1_token_rows,
};
use anyhow::Result;
use rusqlite::Transaction;

/// Migrate legacy claim keys atomically while preserving existing generations.
/// (Migrate khóa claim cũ nguyên tử và giữ nguyên generation hiện có.)
pub(in crate::database::database::schema) fn migrate_legacy_token_schema(
    tx: &Transaction,
) -> Result<()> {
    if !generation_token_migration_needed(tx)? {
        return Ok(());
    }

    let legacy_has_generation_column = legacy_generation_column_present(tx)?;
    create_v2_token_tables(tx)?;
    if legacy_has_generation_column {
        migrate_v1_token_rows(tx)?;
    } else {
        migrate_v0_token_rows(tx)?;
    }
    Ok(())
}

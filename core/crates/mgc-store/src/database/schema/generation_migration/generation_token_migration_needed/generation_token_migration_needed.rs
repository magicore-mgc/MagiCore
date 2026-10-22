//! Detect whether the CAS claim table needs token-key or foreign-key migration.
//! Phát hiện bảng CAS claim cần migrate khóa token hoặc foreign key hay không.
use anyhow::Result;
use rusqlite::Transaction;
use std::collections::HashMap;

/// Return true when the claim key or generation foreign key is incomplete.
/// (Trả true khi khóa claim hoặc foreign key generation chưa đầy đủ.)
pub(in crate::database::database::schema::generation_migration) fn generation_token_migration_needed(
    tx: &Transaction,
) -> Result<bool> {
    let mut generation_in_primary_key = false;
    let mut columns = tx.prepare("PRAGMA table_info(cas_blob_refs)")?;
    let rows = columns.query_map([], |row| {
        Ok((row.get::<_, String>(1)?, row.get::<_, i64>(5)?))
    })?;
    for column in rows {
        let (name, primary_key_position) = column?;
        if name == "generation" && primary_key_position > 0 {
            generation_in_primary_key = true;
        }
    }
    drop(columns);

    Ok(!generation_in_primary_key || !has_generation_foreign_key(tx)?)
}

fn has_generation_foreign_key(tx: &Transaction) -> Result<bool> {
    let mut statement = tx.prepare("PRAGMA foreign_key_list(cas_blob_refs)")?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(6)?,
        ))
    })?;
    let mut composite_columns = HashMap::<i64, (bool, bool, usize)>::new();
    for row in rows {
        let (foreign_key_id, sequence, table, from, to, on_delete) = row?;
        if table != "cas_generations" || on_delete != "CASCADE" {
            continue;
        }
        let columns = composite_columns.entry(foreign_key_id).or_default();
        columns.2 += 1;
        match (sequence, from.as_str(), to.as_str()) {
            (0, "project_root", "project_root") => columns.0 = true,
            (1, "generation", "generation") => columns.1 = true,
            _ => {}
        }
    }
    Ok(composite_columns
        .values()
        .any(|(project_root, generation, part_count)| {
            *project_root && *generation && *part_count == 2
        }))
}

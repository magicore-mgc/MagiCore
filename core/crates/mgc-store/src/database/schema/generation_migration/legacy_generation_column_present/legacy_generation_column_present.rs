//! Detect the pre-migration claim table's generation-column shape.
//! Phát hiện trạng thái cột generation của bảng claim trước migration.
use anyhow::Result;
use rusqlite::Transaction;

/// Return whether legacy claim rows already carry generation values.
/// (Trả về việc row claim cũ đã có giá trị generation hay chưa.)
pub(in crate::database::database::schema::generation_migration) fn legacy_generation_column_present(
    tx: &Transaction,
) -> Result<bool> {
    let legacy_has_generation_column = {
        let mut stmt = tx.prepare("PRAGMA table_info(cas_blob_refs)")?;
        let mut has = false;
        let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;
        for name in rows {
            if name? == "generation" {
                has = true;
            }
        }
        has
    };
    Ok(legacy_has_generation_column)
}

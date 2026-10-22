//! Add one lease column and tolerate only the duplicate-column race.
//! Thêm một cột lease và chỉ dung thứ race trùng tên cột.
use anyhow::Result;
use rusqlite::Connection;

/// Add a lease column, tolerating ONLY the "duplicate column" race from a
/// concurrent migration. Every other error (disk I/O, locked, corrupt)
/// propagates — a store that cannot gain its lease schema must not open
/// half-migrated (fail-closed, Gate 11-B P0-3).
/// (Thêm cột lease, chỉ dung thứ lỗi "duplicate column" từ migration song
/// song. Mọi lỗi khác (I/O đĩa, khóa, hỏng) propagate — store không lấy
/// được schema lease thì không được mở nửa chừng (fail-closed, P0-3).)
pub(in crate::database::database::schema) fn add_lease_column(
    conn: &Connection,
    column: &str,
    ddl: &str,
) -> Result<()> {
    match conn.execute(ddl, []) {
        Ok(_) => Ok(()),
        Err(e) if e.to_string().contains("duplicate column name") => Ok(()),
        Err(e) => Err(anyhow::anyhow!(
            "store.db schema migration failed: unable to add column {column}: {e}"
        )),
    }
}

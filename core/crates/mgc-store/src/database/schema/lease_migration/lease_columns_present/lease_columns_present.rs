//! Read lease-column presence from SQLite schema metadata.
//! Đọc trạng thái cột lease từ metadata schema SQLite.
use anyhow::Result;
use rusqlite::Connection;

/// Read the lease-column shape with one SQLite schema query. These columns
/// are checked when opening a store; probing separately for each one doubled
/// schema I/O on this hot initialization path.
/// (Đọc trạng thái cột lease bằng một truy vấn schema SQLite. Các cột này
/// được kiểm tra khi mở store; thăm dò riêng từng cột làm gấp đôi I/O schema.)
pub(in crate::database::database::schema) fn lease_columns_present(
    conn: &Connection,
) -> Result<(bool, bool)> {
    let mut stmt = conn.prepare("PRAGMA table_info(cas_generations)")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;
    let mut lease_pid = false;
    let mut lease_started_at = false;
    for name in rows {
        match name?.as_str() {
            "lease_pid" => lease_pid = true,
            "lease_started_at" => lease_started_at = true,
            _ => {}
        }
    }
    Ok((lease_pid, lease_started_at))
}

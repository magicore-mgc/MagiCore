use super::super::schema::STORE_DB_BUSY_TIMEOUT;
use anyhow::Result;
use rusqlite::Connection;

pub(in crate::database::database::schema) fn enable_wal_mode(conn: &Connection) -> Result<()> {
    let deadline = std::time::Instant::now() + STORE_DB_BUSY_TIMEOUT;
    let mut backoff = std::time::Duration::from_millis(5);
    loop {
        match conn.query_row("PRAGMA journal_mode=WAL", [], |row| row.get::<_, String>(0)) {
            Ok(mode) if mode.eq_ignore_ascii_case("wal") => return Ok(()),
            Ok(mode) => anyhow::bail!(
                "store.db refused WAL mode (SQLite selected {mode:?}); concurrent store access requires WAL"
            ),
            Err(rusqlite::Error::SqliteFailure(error, _))
                if matches!(
                    error.code,
                    rusqlite::ErrorCode::DatabaseBusy | rusqlite::ErrorCode::DatabaseLocked
                ) && std::time::Instant::now() < deadline =>
            {
                std::thread::sleep(backoff);
                backoff = (backoff * 2).min(std::time::Duration::from_millis(100));
            }
            Err(error) => return Err(error.into()),
        }
    }
}

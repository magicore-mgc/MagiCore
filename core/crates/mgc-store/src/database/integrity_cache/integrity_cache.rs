use super::super::{numeric::sqlite_i64, types::Database};
use anyhow::Result;
use rusqlite::params;

impl Database {
    pub fn verify_integrity(&self, hash: &str) -> Result<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM integrity_cache WHERE hash = ?1")?;
        let count: i64 = stmt.query_row(params![hash], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn cache_integrity(&self, hash: &str) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let now = sqlite_i64(now)?;

        self.conn.execute(
            "INSERT OR REPLACE INTO integrity_cache (hash, verified_at) VALUES (?1, ?2)",
            params![hash, now],
        )?;
        Ok(())
    }
}

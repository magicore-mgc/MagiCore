use super::super::{
    numeric::{sqlite_i64, u64_from_sqlite},
    types::Database,
};
use anyhow::Result;
use rusqlite::params;

impl Database {
    /// Min-release-age (seconds) for an ecosystem. None = not set (use default).
    pub fn release_policy(&self, ecosystem: &str) -> Result<Option<u64>> {
        let mut stmt = self
            .conn
            .prepare("SELECT min_age_secs FROM release_policy WHERE ecosystem = ?1")?;
        let mut rows = stmt.query_map(params![ecosystem], |row| u64_from_sqlite(row.get(0)?, 0))?;
        match rows.next() {
            Some(Ok(v)) => Ok(Some(v)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    pub fn upsert_release_policy(&self, ecosystem: &str, min_age_secs: u64) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO release_policy (ecosystem, min_age_secs) VALUES (?1, ?2)",
            params![ecosystem, sqlite_i64(min_age_secs)?],
        )?;
        Ok(())
    }
}

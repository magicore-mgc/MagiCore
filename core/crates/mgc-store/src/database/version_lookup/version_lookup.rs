use super::super::types::Database;
use anyhow::Result;
use mgc_types::Version;
use rusqlite::params;

impl Database {
    /// Highest installed version for a package name — None = not installed.
    /// Used by the T5 no-downgrade guard (semver compare, not string compare).
    pub fn latest_installed_version(&self, name: &str) -> Result<Option<Version>> {
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT version FROM packages WHERE id = ?1")?;
        let rows = stmt.query_map(params![name], |row| row.get::<_, String>(0))?;
        let mut best: Option<Version> = None;
        for row in rows {
            let v = match Version::parse(&row?) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if best.as_ref().is_none_or(|b| v > *b) {
                best = Some(v);
            }
        }
        Ok(best)
    }
}

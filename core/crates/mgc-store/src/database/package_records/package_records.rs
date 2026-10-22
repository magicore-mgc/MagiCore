use super::super::{
    numeric::{sqlite_i64, u64_from_sqlite},
    types::{Database, DatabaseEntry},
};
use anyhow::Result;
use mgc_types::PackageId;
use rusqlite::params;

impl Database {
    pub fn insert_package(&self, id: &PackageId, integrity: Option<&str>) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let now = sqlite_i64(now)?;

        self.conn.execute(
        "INSERT OR REPLACE INTO packages (id, version, integrity, installed_at) VALUES (?1, ?2, ?3, ?4)",
        params![id.name_str(), id.version().to_string(), integrity, now],
    )?;

        Ok(())
    }

    pub fn is_installed(&self, id: &PackageId) -> Result<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM packages WHERE id = ?1 AND version = ?2")?;
        let count: i64 = stmt
            .query_row(params![id.name_str(), id.version().to_string()], |row| {
                row.get(0)
            })?;
        Ok(count > 0)
    }

    pub fn list_installed(&self) -> Result<Vec<DatabaseEntry>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, version, integrity, installed_at FROM packages ORDER BY id")?;
        let entries = stmt.query_map([], |row| {
            Ok(DatabaseEntry {
                id: row.get(0)?,
                version: row.get(1)?,
                integrity: row.get(2)?,
                installed_at: u64_from_sqlite(row.get(3)?, 3)?,
            })
        })?;

        let mut result = Vec::new();
        for entry in entries {
            result.push(entry?);
        }
        Ok(result)
    }

    pub fn remove_package(&self, id: &PackageId) -> Result<()> {
        self.conn.execute(
            "DELETE FROM packages WHERE id = ?1 AND version = ?2",
            params![id.name_str(), id.version().to_string()],
        )?;
        Ok(())
    }
}

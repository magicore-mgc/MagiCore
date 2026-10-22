use super::super::{
    numeric::{sqlite_i64, u64_from_sqlite},
    types::Database,
};
use anyhow::Result;
use rusqlite::params;

impl Database {
    /// Policy recorded for a package. `package_id` is the raw key: `name` or
    /// `name@version`.
    pub fn get_trust_policy(&self, package_id: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT policy FROM trust_policy WHERE package_id = ?1")?;
        let mut rows = stmt.query_map(params![package_id], |row| row.get::<_, String>(0))?;
        match rows.next() {
            Some(Ok(policy)) => Ok(Some(policy)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    /// Record approve/deny for a package (`name` covers all versions,
    /// `name@version` covers exactly one).
    pub fn upsert_trust_policy(&self, package_id: &str, policy: &str) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let now = sqlite_i64(now)?;
        self.conn.execute(
        "INSERT OR REPLACE INTO trust_policy (package_id, policy, updated_at) VALUES (?1, ?2, ?3)",
        params![package_id, policy, now],
    )?;
        Ok(())
    }

    pub fn list_trust_policies(&self) -> Result<Vec<(String, String, u64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT package_id, policy, updated_at FROM trust_policy ORDER BY package_id",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, u64_from_sqlite(row.get(2)?, 2)?))
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    pub fn clear_trust_policy(&self, package_id: &str) -> Result<bool> {
        let n = self.conn.execute(
            "DELETE FROM trust_policy WHERE package_id = ?1",
            params![package_id],
        )?;
        Ok(n > 0)
    }

    /// Remove policies for packages no longer installed (bare name or
    /// name@version). Returns number of rows removed. `mgc trust prune`.
    pub fn prune_trust_policies(&self) -> Result<usize> {
        let n = self.conn.execute(
            "DELETE FROM trust_policy WHERE package_id NOT IN (
                 SELECT id || '@' || version FROM packages
             ) AND package_id NOT IN (
                 SELECT id FROM packages
             )",
            [],
        )?;
        Ok(n)
    }
}

use super::super::{
    numeric::{sqlite_i64, u64_from_sqlite},
    types::Database,
};
use anyhow::Result;
use mgc_types::PackageId;
use rusqlite::params;

impl Database {
    /// Replace the file listing for one package (schema + source of truth for
    /// StoreIndex). Atomic within a transaction.
    pub fn replace_package_files(
        &self,
        id: &PackageId,
        files: &[(String, String, u64)],
    ) -> Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        tx.execute(
            "DELETE FROM package_files WHERE id = ?1 AND version = ?2",
            params![id.name_str(), id.version().to_string()],
        )?;
        {
            let mut stmt = tx.prepare(
            "INSERT INTO package_files (id, version, path, blob_hash, size) VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
            for (path, hash, size) in files {
                stmt.execute(params![
                    id.name_str(),
                    id.version().to_string(),
                    path,
                    hash,
                    sqlite_i64(*size)?
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    /// Full file listing for a package — ordered by path. Empty = not indexed.
    pub fn list_package_files(&self, id: &PackageId) -> Result<Vec<(String, String, u64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT path, blob_hash, size FROM package_files
             WHERE id = ?1 AND version = ?2 ORDER BY path",
        )?;
        let rows = stmt.query_map(params![id.name_str(), id.version().to_string()], |row| {
            Ok((row.get(0)?, row.get(1)?, u64_from_sqlite(row.get(2)?, 2)?))
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Every blob hash referenced by any indexed package.
    pub fn list_all_blob_hashes(&self) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT blob_hash FROM package_files ORDER BY blob_hash")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Number of indexed packages — cheap health check for StoreIndex.rebuild.
    pub fn count_indexed_packages(&self) -> Result<usize> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(DISTINCT id || '#' || version) FROM package_files",
            [],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

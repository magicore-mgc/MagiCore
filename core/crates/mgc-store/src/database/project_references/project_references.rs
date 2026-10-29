use super::super::types::Database;
use anyhow::Result;
use mgc_types::PackageId;
use rusqlite::params;

impl Database {
    /// Mark (project_root, package_id) as referenced — install. (02 §2.2)
    pub fn set_ref(&self, project_root: &str, id: &PackageId) -> Result<()> {
        self.conn.execute(
            "INSERT INTO refs (project_root, package_id, ref_count) VALUES (?1, ?2, 1)
             ON CONFLICT(project_root, package_id)
             DO UPDATE SET ref_count = 1",
            params![project_root, id.to_string()],
        )?;
        Ok(())
    }

    /// Remove the project reference for a package — remove/uninstall.
    pub fn clear_ref(&self, project_root: &str, id: &PackageId) -> Result<()> {
        self.conn.execute(
            "DELETE FROM refs WHERE project_root = ?1 AND package_id = ?2",
            params![project_root, id.to_string()],
        )?;
        Ok(())
    }

    /// Remove all references of a project — called before re-install so the
    /// refs table mirrors the current graph exactly (no stale entries).
    pub fn clear_all_refs(&self, project_root: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM refs WHERE project_root = ?1",
            params![project_root],
        )?;
        Ok(())
    }

    /// Return installed packages with no project referencing them —
    /// candidates for `mgc store prune` (02 §2.2).
    pub fn list_unreferenced(&self) -> Result<Vec<PackageId>> {
        let mut stmt = self.conn.prepare(
            "SELECT p.id, p.version FROM packages p
             LEFT JOIN refs r ON r.package_id = p.id || '@' || p.version
             GROUP BY p.id, p.version
             HAVING COUNT(r.project_root) = 0",
        )?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let version: String = row.get(1)?;
            Ok(format!("{id}@{version}"))
        })?;
        let mut result = Vec::new();
        for row in rows {
            if let Ok(parsed) = PackageId::parse(&row?) {
                result.push(parsed);
            }
        }
        Ok(result)
    }
}

impl Database {
    /// Set the full reference set of one project in a single transaction.
    /// Đặt toàn bộ tập tham chiếu của một project trong một transaction.
    ///
    /// Clearing then re-inserting one row at a time left a window where the
    /// project referenced nothing: a concurrent `mgc store prune` reading that
    /// window would delete every blob the project still needs (P-2.4).
    /// (Xoá rồi chèn lại từng dòng để lộ cửa sổ project không tham chiếu gì:
    /// `mgc store prune` chạy đồng thời đọc cửa sổ đó sẽ xoá mọi blob project
    /// còn cần — P-2.4.)
    pub fn replace_refs<'a, I>(&self, project_root: &str, ids: I) -> Result<u64>
    where
        I: IntoIterator<Item = &'a PackageId>,
    {
        // The delete and the inserts must run inside one transaction on the
        // database's own connection, as the neighbouring multi-row writers do.
        // (DELETE và INSERT phải chạy trong một transaction trên connection của
        // chính database, như các writer nhiều dòng bên cạnh.)
        let transaction = self.conn.unchecked_transaction()?;
        transaction.execute(
            "DELETE FROM refs WHERE project_root = ?1",
            params![project_root],
        )?;
        let mut written = 0u64;
        {
            let mut insert = transaction.prepare(
                "INSERT INTO refs (project_root, package_id, ref_count) VALUES (?1, ?2, 1)
                 ON CONFLICT(project_root, package_id) DO UPDATE SET ref_count = 1",
            )?;
            for id in ids {
                insert.execute(params![project_root, id.to_string()])?;
                written += 1;
            }
        }
        transaction.commit()?;
        Ok(written)
    }

    /// How many projects reference a package — the refcount `prune` honours.
    /// Bao nhiêu project tham chiếu một package — refcount mà `prune` tôn trọng.
    pub fn reference_count(&self, id: &PackageId) -> Result<u64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(DISTINCT project_root) FROM refs WHERE package_id = ?1",
            params![id.to_string()],
            |row| row.get(0),
        )?;
        Ok(count.max(0) as u64)
    }
}

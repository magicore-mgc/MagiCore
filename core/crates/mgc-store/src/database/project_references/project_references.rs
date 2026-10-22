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

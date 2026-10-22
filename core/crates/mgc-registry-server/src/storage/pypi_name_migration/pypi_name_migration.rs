//! Canonicalizes legacy PyPI project identities transactionally and keeps the one-time migration marker.
//! Chuẩn hóa định danh project PyPI cũ trong một transaction và giữ marker migration một lần.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    pub(in crate::storage) async fn migrate_pypi_project_names(&self) -> Result<()> {
        const MIGRATION: &str = "canonical-pypi-project-names-v1";
        let mut tx = self.db.begin().await?;
        let already_applied = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM mgc_runtime_migrations WHERE name = ?)",
        )
        .bind(MIGRATION)
        .fetch_one(&mut *tx)
        .await?;
        if already_applied != 0 {
            tx.rollback().await?;
            return Ok(());
        }

        let rows = sqlx::query(
            "SELECT name, version, filename, digest, size, requires_python, created_at FROM pypi_files ORDER BY rowid",
        )
        .fetch_all(&mut *tx)
        .await?;
        let mut files: std::collections::BTreeMap<
            (String, String),
            (crate::model::PypiFile, String),
        > = std::collections::BTreeMap::new();
        for row in rows {
            let original_name = row.get::<String, _>("name");
            let name = crate::trusted::canonical_pypi_name(&original_name)
                .unwrap_or_else(|_| original_name.clone());
            let file = crate::model::PypiFile {
                name: name.clone(),
                version: row.get("version"),
                filename: row.get("filename"),
                digest: row.get("digest"),
                size: row.get("size"),
                requires_python: row.get("requires_python"),
            };
            let key = (name, file.filename.clone());
            let created_at: String = row.get("created_at");
            if let Some((existing, _)) = files.get(&key) {
                if existing.version != file.version
                    || existing.digest != file.digest
                    || existing.size != file.size
                    || existing.requires_python != file.requires_python
                {
                    anyhow::bail!(
                        "legacy PyPI aliases contain conflicting file {}; resolve the duplicate before starting the registry",
                        file.filename
                    );
                }
            } else {
                files.insert(key, (file, created_at));
            }
        }

        sqlx::query("DELETE FROM pypi_files")
            .execute(&mut *tx)
            .await?;
        for (_, (file, created_at)) in files {
            sqlx::query(
                "INSERT INTO pypi_files (name, version, filename, digest, size, requires_python, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&file.name)
            .bind(&file.version)
            .bind(&file.filename)
            .bind(&file.digest)
            .bind(file.size)
            .bind(&file.requires_python)
            .bind(created_at)
            .execute(&mut *tx)
            .await?;
        }
        sqlx::query("INSERT INTO mgc_runtime_migrations (name) VALUES (?)")
            .bind(MIGRATION)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/pypi_name_migration.rs"]
mod tests;

//! Looks up and stores the existing PyPI file index entries.
//! Truy vấn và lưu các entry index file PyPI hiện có.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    pub async fn get_pypi_file_digest(&self, name: &str, filename: &str) -> Result<Option<String>> {
        let name = crate::trusted::canonical_pypi_name(name)
            .map_err(|_| anyhow::anyhow!("invalid PyPI project name"))?;
        let row = sqlx::query("SELECT digest FROM pypi_files WHERE name = ? AND filename = ?")
            .bind(name)
            .bind(filename)
            .fetch_optional(&self.db)
            .await?;
        Ok(row.map(|r| r.get("digest")))
    }

    pub async fn get_pypi_files(&self, name: &str) -> Result<Vec<crate::model::PypiFile>> {
        let name = crate::trusted::canonical_pypi_name(name)
            .map_err(|_| anyhow::anyhow!("invalid PyPI project name"))?;
        let rows = sqlx::query(
            r#"
            SELECT name, version, filename, digest, size, requires_python
            FROM pypi_files WHERE name = ? ORDER BY filename
        "#,
        )
        .bind(name)
        .fetch_all(&self.db)
        .await?;
        let mut out = Vec::new();
        for row in rows {
            out.push(crate::model::PypiFile {
                name: row.get("name"),
                version: row.get("version"),
                filename: row.get("filename"),
                digest: row.get("digest"),
                size: row.get("size"),
                requires_python: row.get("requires_python"),
            });
        }
        Ok(out)
    }

    pub async fn put_pypi_file(&self, file: &crate::model::PypiFile) -> Result<()> {
        let name = crate::trusted::canonical_pypi_name(&file.name)
            .map_err(|_| anyhow::anyhow!("invalid PyPI project name"))?;
        sqlx::query(
            r#"
            INSERT INTO pypi_files (name, version, filename, digest, size, requires_python)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(name, filename) DO UPDATE SET
                version = excluded.version,
                digest = excluded.digest,
                size = excluded.size,
                requires_python = excluded.requires_python
        "#,
        )
        .bind(name)
        .bind(&file.version)
        .bind(&file.filename)
        .bind(&file.digest)
        .bind(file.size)
        .bind(&file.requires_python)
        .execute(&self.db)
        .await?;
        Ok(())
    }
}

//! Owns package writes and deletion operations.
//! Sở hữu thao tác ghi và xóa package.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    pub async fn put_package(&self, pkg: &crate::model::Package) -> Result<()> {
        let mut tx = self.db.begin().await?;
        let binding = sqlx::query_scalar::<_, String>(
            "SELECT repository FROM trusted_publishers WHERE package = ?",
        )
        .bind(&pkg.name)
        .fetch_optional(&mut *tx)
        .await?;
        if binding.is_some() {
            anyhow::bail!("package requires trusted OIDC publishing");
        }
        let time_json = serde_json::to_string(&pkg.time)?;
        let dist_tags_json = serde_json::to_string(&pkg.dist_tags)?;
        let maintainers_json = serde_json::to_string(&pkg.maintainers)?;

        sqlx::query(
            r#"
            INSERT INTO packages (name, description, dist_tags, maintainers, time, private)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(name) DO UPDATE SET
                description = excluded.description,
                dist_tags = excluded.dist_tags,
                maintainers = excluded.maintainers,
                time = excluded.time,
                private = excluded.private,
                updated_at = datetime('now')
        "#,
        )
        .bind(&pkg.name)
        .bind(&pkg.description)
        .bind(&dist_tags_json)
        .bind(&maintainers_json)
        .bind(&time_json)
        .bind(pkg.private)
        .execute(&mut *tx)
        .await?;

        // Save versions
        for (version, ver) in &pkg.versions {
            let _ver_json = serde_json::to_string(ver)?;
            let id = format!("{}@{}", pkg.name, version);
            sqlx::query(
                r#"
                INSERT INTO package_versions (id, package_name, version, data)
                VALUES (?, ?, ?, ?)
                ON CONFLICT(id) DO UPDATE SET data = excluded.data
            "#,
            )
            .bind(&id)
            .bind(&pkg.name)
            .bind(version)
            .bind(serde_json::to_string(ver)?)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn delete_package(&self, name: &str) -> Result<()> {
        sqlx::query("DELETE FROM packages WHERE name = ?")
            .bind(name)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    /// Xóa 1 version của package; trả true nếu version tồn tại
    /// (dist-tags trỏ tới version bị xóa cũng bị bỏ — npm behavior)
    pub async fn delete_package_version(&self, name: &str, version: &str) -> Result<bool> {
        let mut pkg = match self.get_package(name).await? {
            Some(p) => p,
            None => return Ok(false),
        };
        if pkg.versions.remove(version).is_none() {
            return Ok(false);
        }
        pkg.dist_tags.retain(|_, v| v != version);
        // Nếu hết version → xóa package luôn; ngược lại lưu lại
        if pkg.versions.is_empty() {
            self.delete_package(name).await?;
        } else {
            self.put_package(&pkg).await?;
        }
        Ok(true)
    }
}

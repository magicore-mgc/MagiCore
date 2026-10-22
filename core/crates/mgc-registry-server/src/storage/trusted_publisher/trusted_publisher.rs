//! Owns trusted publisher bindings and trusted distribution-tag updates.
//! Sở hữu liên kết trusted publisher và cập nhật distribution tag tin cậy.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    pub async fn set_trusted_publisher(&self, package: &str, repository: &str) -> Result<()> {
        let mut tx = self.db.begin().await?;
        sqlx::query(
            "INSERT INTO trusted_publishers (package, repository, binding_generation) VALUES (?, ?, 1) ON CONFLICT(package) DO UPDATE SET repository = excluded.repository, binding_generation = trusted_publishers.binding_generation + CASE WHEN trusted_publishers.repository != excluded.repository THEN 1 ELSE 0 END, updated_at = datetime('now')",
        )
        .bind(package)
        .bind(repository)
        .execute(&mut *tx)
        .await?;
        sqlx::query(
            "INSERT INTO audit_log (event_type, name, version, user) VALUES (?, ?, NULL, ?)",
        )
        .bind("trusted-publisher-bound")
        .bind(package)
        .bind(repository)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn trusted_publisher(&self, package: &str) -> Result<Option<String>> {
        let row = sqlx::query("SELECT repository FROM trusted_publishers WHERE package = ?")
            .bind(package)
            .fetch_optional(&self.db)
            .await?;
        Ok(row.map(|row| row.get("repository")))
    }

    pub async fn trusted_publisher_binding(&self, package: &str) -> Result<Option<(String, u64)>> {
        let row = sqlx::query(
            "SELECT repository, binding_generation FROM trusted_publishers WHERE package = ?",
        )
        .bind(package)
        .fetch_optional(&self.db)
        .await?;
        row.map(|row| {
            let generation = u64::try_from(row.get::<i64, _>("binding_generation"))
                .context("trusted package binding generation is invalid")?;
            Ok((row.get("repository"), generation))
        })
        .transpose()
    }

    pub async fn set_trusted_dist_tag(
        &self,
        package: &str,
        tag: &str,
        version: &str,
        expected_binding_generation: u64,
    ) -> Result<bool> {
        let mut tx = self.db.begin().await?;
        let Some(binding) =
            sqlx::query("SELECT binding_generation FROM trusted_publishers WHERE package = ?")
                .bind(package)
                .fetch_optional(&mut *tx)
                .await?
        else {
            return Ok(false);
        };
        let binding_generation = u64::try_from(binding.get::<i64, _>("binding_generation"))
            .context("trusted package binding generation is invalid")?;
        if binding_generation != expected_binding_generation {
            return Ok(false);
        }
        let has_version: i64 = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM package_versions WHERE package_name = ? AND version = ?)",
        )
        .bind(package)
        .bind(version)
        .fetch_one(&mut *tx)
        .await?;
        if has_version == 0 {
            return Ok(false);
        }
        let Some(stored_tags) =
            sqlx::query_scalar::<_, String>("SELECT dist_tags FROM packages WHERE name = ?")
                .bind(package)
                .fetch_optional(&mut *tx)
                .await?
        else {
            return Ok(false);
        };
        let mut tags: HashMap<String, String> = serde_json::from_str(&stored_tags)?;
        tags.insert(tag.to_string(), version.to_string());
        sqlx::query(
            "UPDATE packages SET dist_tags = ?, updated_at = datetime('now') WHERE name = ?",
        )
        .bind(serde_json::to_string(&tags)?)
        .bind(package)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(true)
    }
}

//! Lists OCI manifest tags and repositories from the existing registry metadata.
//! Liệt kê tag manifest OCI và repository từ metadata registry hiện có.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    pub async fn list_oci_tags(&self, repo: &str) -> Result<Vec<String>> {
        let rows = sqlx::query_scalar::<_, String>(
            "SELECT reference FROM oci_manifests WHERE repo = ? AND reference NOT LIKE 'sha256:%'",
        )
        .bind(repo)
        .fetch_all(&self.db)
        .await?;
        Ok(rows)
    }

    pub async fn list_oci_repos(&self) -> Result<Vec<String>> {
        let rows = sqlx::query(
            "SELECT DISTINCT repo FROM (SELECT repo FROM oci_manifests UNION SELECT repo FROM oci_blobs)"
        )
        .fetch_all(&self.db)
        .await?;
        Ok(rows.iter().map(|r| r.get("repo")).collect())
    }
}

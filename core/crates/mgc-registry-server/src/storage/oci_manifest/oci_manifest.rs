//! Owns OCI manifest storage, raw manifest lookup, and deletion by tag or digest.
//! Sở hữu lưu OCI manifest, truy vấn manifest thô và xóa theo tag hoặc digest.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    pub async fn put_oci_manifest(
        &self,
        repo: &str,
        reference: &str,
        manifest_bytes: &[u8],
    ) -> Result<()> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(manifest_bytes);
        let digest = format!("sha256:{}", hex::encode(hasher.finalize()));
        let manifest_json = String::from_utf8_lossy(manifest_bytes);
        sqlx::query(r#"
            INSERT INTO oci_manifests (repo, reference, manifest, digest)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(repo, reference) DO UPDATE SET manifest = excluded.manifest, digest = excluded.digest
        "#)
        .bind(repo)
        .bind(reference)
        .bind(manifest_json.as_ref())
        .bind(digest)
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn get_oci_manifest(
        &self,
        repo: &str,
        reference: &str,
    ) -> Result<Option<crate::model::OciManifest>> {
        let row =
            sqlx::query("SELECT manifest FROM oci_manifests WHERE repo = ? AND reference = ?")
                .bind(repo)
                .bind(reference)
                .fetch_optional(&self.db)
                .await?;

        if let Some(row) = row {
            let manifest_json: String = row.get("manifest");
            let manifest = serde_json::from_str(&manifest_json)?;
            return Ok(Some(manifest));
        }

        // Fallback: reference may be a content digest
        // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
        if let Some(digest) = reference.strip_prefix("sha256:")
            && let Some(row) =
                sqlx::query("SELECT manifest FROM oci_manifests WHERE repo = ? AND digest = ?")
                    .bind(repo)
                    .bind(format!("sha256:{}", digest))
                    .fetch_optional(&self.db)
                    .await?
        {
            let manifest_json: String = row.get("manifest");
            let manifest = serde_json::from_str(&manifest_json)?;
            return Ok(Some(manifest));
        }
        Ok(None)
    }

    /// Raw manifest bytes + stored content digest (as pushed by the client).
    pub async fn get_oci_manifest_raw(
        &self,
        repo: &str,
        reference: &str,
    ) -> Result<Option<(String, String)>> {
        let row = sqlx::query(
            "SELECT manifest, digest FROM oci_manifests WHERE repo = ? AND reference = ?",
        )
        .bind(repo)
        .bind(reference)
        .fetch_optional(&self.db)
        .await?;

        if let Some(row) = row {
            return Ok(Some((row.get("manifest"), row.get("digest"))));
        }

        // Fallback: reference may be a content digest
        if let Some(digest) = reference.strip_prefix("sha256:") {
            let row = sqlx::query(
                "SELECT manifest, digest FROM oci_manifests WHERE repo = ? AND digest = ?",
            )
            .bind(repo)
            .bind(format!("sha256:{}", digest))
            .fetch_optional(&self.db)
            .await?;
            if let Some(row) = row {
                return Ok(Some((row.get("manifest"), row.get("digest"))));
            }
        }
        Ok(None)
    }

    pub async fn delete_oci_manifest(&self, repo: &str, reference: &str) -> Result<()> {
        sqlx::query("DELETE FROM oci_manifests WHERE repo = ? AND reference = ?")
            .bind(repo)
            .bind(reference)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}

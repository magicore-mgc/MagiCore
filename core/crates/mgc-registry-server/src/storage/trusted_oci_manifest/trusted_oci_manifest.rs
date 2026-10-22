//! Stores a trusted OCI manifest with its existing attestation linkage.
//! Lưu trusted OCI manifest cùng liên kết attestation hiện có.

use super::super::prelude::*;
use super::super::*;

/// OCI manifest bytes and the trusted attestation committed with them.
/// Byte manifest OCI và attestation trusted được commit cùng transaction.
pub struct TrustedOciManifestInput<'a> {
    pub repo: &'a str,
    pub reference: &'a str,
    pub manifest_bytes: &'a [u8],
    pub attestation: TrustedAttestationInput<'a>,
}

impl RegistryStore {
    pub async fn put_trusted_oci_manifest(&self, input: TrustedOciManifestInput<'_>) -> Result<()> {
        let TrustedOciManifestInput {
            repo,
            reference,
            manifest_bytes,
            attestation,
        } = input;
        use sha2::{Digest, Sha256};
        let actual_sha256 = hex::encode(Sha256::digest(manifest_bytes));
        if actual_sha256 != attestation.artifact_sha256
            || attestation.registry_digest != format!("sha256:{actual_sha256}")
        {
            anyhow::bail!("trusted OCI manifest digest does not match its bytes");
        }
        let manifest_json = String::from_utf8(manifest_bytes.to_vec())
            .context("OCI manifest is not valid UTF-8")?;
        let mut tx = self.db.begin().await?;
        sqlx::query(
            r#"INSERT INTO oci_manifests (repo, reference, manifest, digest)
               VALUES (?, ?, ?, ?)
               ON CONFLICT(repo, reference) DO UPDATE SET manifest = excluded.manifest,
               digest = excluded.digest"#,
        )
        .bind(repo)
        .bind(reference)
        .bind(manifest_json)
        .bind(attestation.registry_digest)
        .execute(&mut *tx)
        .await?;
        Self::append_trusted_attestation_tx(&mut tx, attestation).await?;
        tx.commit().await?;
        Ok(())
    }
}

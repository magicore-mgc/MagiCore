//! Stores a PyPI file after the existing trusted-publisher validation path.
//! Lưu file PyPI sau luồng xác thực trusted-publisher hiện có.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    /// Store a trusted PyPI file and its Sigstore record atomically.
    /// Lưu file PyPI trusted cùng bản ghi Sigstore trong cùng transaction.
    pub async fn put_trusted_pypi_file(
        &self,
        file: &crate::model::PypiFile,
        claims: &mgc_oidc::OidcClaims,
        key: &mgc_crypto::KeyPair,
        expected_binding_generation: u64,
        artifact_sha256: &str,
        sigstore_bundle: serde_json::Value,
    ) -> Result<()> {
        let mut canonical_file = file.clone();
        canonical_file.name = crate::trusted::canonical_pypi_name(&file.name)
            .map_err(|_| anyhow::anyhow!("invalid PyPI project name"))?;
        let package = crate::trusted::scoped_package("pypi", &canonical_file.name)
            .map_err(|_| anyhow::anyhow!("invalid PyPI project name"))?;
        let mut tx = self.db.begin().await?;
        sqlx::query(
            r#"INSERT INTO pypi_files (name, version, filename, digest, size, requires_python)
               VALUES (?, ?, ?, ?, ?, ?)
               ON CONFLICT(name, filename) DO UPDATE SET version = excluded.version,
               digest = excluded.digest, size = excluded.size, requires_python = excluded.requires_python"#,
        )
        .bind(&canonical_file.name)
        .bind(&canonical_file.version)
        .bind(&canonical_file.filename)
        .bind(&canonical_file.digest)
        .bind(canonical_file.size)
        .bind(&canonical_file.requires_python)
        .execute(&mut *tx)
        .await?;
        Self::append_trusted_attestation_tx(
            &mut tx,
            TrustedAttestationInput {
                package: &package,
                version: &file.version,
                registry_digest: &file.digest,
                artifact_sha256,
                claims,
                key,
                expected_binding_generation,
                sigstore_bundle,
            },
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }
}

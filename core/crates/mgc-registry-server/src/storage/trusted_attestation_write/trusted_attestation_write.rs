//! Owns trusted attestation insertion and the transaction helper used by trusted package writes.
//! Sở hữu ghi trusted attestation và helper transaction được trusted package sử dụng.

use super::super::prelude::*;
use super::super::*;

/// Inputs bound together when persisting a trusted publication record.
/// Dữ liệu được ràng buộc cùng nhau khi lưu một bản ghi phát hành trusted.
pub struct TrustedAttestationInput<'a> {
    pub package: &'a str,
    pub version: &'a str,
    pub registry_digest: &'a str,
    pub artifact_sha256: &'a str,
    pub claims: &'a mgc_oidc::OidcClaims,
    pub key: &'a mgc_crypto::KeyPair,
    pub expected_binding_generation: u64,
    pub sigstore_bundle: serde_json::Value,
}

impl RegistryStore {
    pub async fn append_trusted_attestation(
        &self,
        attestation: TrustedAttestationInput<'_>,
    ) -> Result<()> {
        let mut tx = self.db.begin().await?;
        Self::append_trusted_attestation_tx(&mut tx, attestation).await?;
        tx.commit().await?;
        Ok(())
    }

    pub(in crate::storage) async fn append_trusted_attestation_tx(
        tx: &mut sqlx::Transaction<'_, Sqlite>,
        attestation: TrustedAttestationInput<'_>,
    ) -> Result<()> {
        let TrustedAttestationInput {
            package,
            version,
            registry_digest,
            artifact_sha256,
            claims,
            key,
            expected_binding_generation,
            sigstore_bundle,
        } = attestation;
        if !crate::trusted::is_valid_sha256_hex(artifact_sha256) {
            anyhow::bail!("trusted publish requires a valid artifact SHA-256 digest");
        }
        let binding = sqlx::query(
            "SELECT repository, binding_generation FROM trusted_publishers WHERE package = ?",
        )
        .bind(package)
        .fetch_optional(&mut **tx)
        .await?;
        let Some(binding) = binding else {
            anyhow::bail!("trusted publish requires an active package binding");
        };
        let generation = u64::try_from(binding.get::<i64, _>("binding_generation"))
            .context("trusted package binding generation is invalid")?;
        if generation != expected_binding_generation {
            anyhow::bail!("trusted package binding changed after token authorization");
        }
        let repository = binding.get::<String, _>("repository");
        if !crate::trusted::claims_match_repository_binding(claims, &repository) {
            anyhow::bail!("trusted publisher claims do not match the active package binding");
        }
        let last = sqlx::query(
            "SELECT sequence, entry_hash FROM trusted_attestations WHERE package = ? ORDER BY sequence DESC LIMIT 1",
        )
        .bind(package)
        .fetch_optional(&mut **tx)
        .await?;
        let previous_sequence = last
            .as_ref()
            .map(|row| row.get::<i64, _>("sequence"))
            .unwrap_or(0);
        if previous_sequence < 0 {
            anyhow::bail!("trusted attestation sequence is invalid");
        }
        let sequence = (previous_sequence as u64)
            .checked_add(1)
            .filter(|next| *next <= i64::MAX as u64)
            .ok_or_else(|| anyhow::anyhow!("trusted attestation sequence is exhausted"))?;
        let previous_hash = last.map(|row| row.get::<String, _>("entry_hash"));
        let payload = crate::trusted::AttestationPayload {
            schema_version: 1,
            package: package.to_owned(),
            version: version.to_owned(),
            digest: registry_digest.to_owned(),
            artifact_sha256: Some(artifact_sha256.to_owned()),
            sequence,
            builder: crate::trusted::BuilderIdentity::from(claims),
            created_at_unix: chrono::Utc::now().timestamp().max(0) as u64,
            previous_hash,
        };
        let mut entry = crate::trusted::sign_attestation(payload, key)?;
        entry.sigstore_bundle = Some(sigstore_bundle.clone());
        sqlx::query(
            "INSERT INTO trusted_attestations (package, sequence, payload, entry_hash, key_id, signature, sigstore_bundle) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(package)
        .bind(sequence as i64)
        .bind(serde_json::to_string(&entry.payload)?)
        .bind(&entry.entry_hash)
        .bind(&entry.key_id)
        .bind(&entry.signature)
        .bind(serde_json::to_string(&sigstore_bundle)?)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
}

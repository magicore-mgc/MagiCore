//! Stores packages with trusted provenance while preserving the existing transaction and publisher checks.
//! Lưu package kèm provenance tin cậy, giữ nguyên transaction và kiểm tra publisher.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    /// Store a package and its signed provenance entries in one SQLite transaction.
    /// Lưu package cùng provenance đã ký trong một transaction SQLite.
    pub async fn put_trusted_package(
        &self,
        pkg: &crate::model::Package,
        claims: &mgc_oidc::OidcClaims,
        key: &mgc_crypto::KeyPair,
        expected_binding_generation: u64,
        public_attestations: &HashMap<String, (String, serde_json::Value)>,
    ) -> Result<()> {
        crate::trusted::scoped_package("npm", &pkg.name)
            .map_err(|_| anyhow::anyhow!("trusted npm package name is invalid"))?;
        if pkg.versions.is_empty() {
            anyhow::bail!("trusted publish must include at least one version");
        }

        let mut tx = self.db.begin().await?;
        let binding = sqlx::query(
            "SELECT repository, binding_generation FROM trusted_publishers WHERE package = ?",
        )
        .bind(&pkg.name)
        .fetch_optional(&mut *tx)
        .await?;
        let Some(repository) = binding else {
            anyhow::bail!("trusted publish requires an active package binding");
        };
        let binding_generation = u64::try_from(repository.get::<i64, _>("binding_generation"))
            .context("trusted package binding generation is invalid")?;
        if binding_generation != expected_binding_generation {
            anyhow::bail!("trusted package binding changed after token authorization");
        }
        let repository = repository.get::<String, _>("repository");
        if !crate::trusted::claims_match_repository_binding(claims, &repository) {
            anyhow::bail!("trusted publisher claims do not match the active package binding");
        }
        let last = sqlx::query(
            "SELECT sequence, entry_hash FROM trusted_attestations WHERE package = ? ORDER BY sequence DESC LIMIT 1",
        )
        .bind(&pkg.name)
        .fetch_optional(&mut *tx)
        .await?;
        let sequence = last
            .as_ref()
            .map(|row| row.get::<i64, _>("sequence"))
            .unwrap_or(0);
        if sequence < 0 {
            anyhow::bail!("trusted attestation sequence is invalid");
        }
        let mut sequence = sequence as u64;
        let mut previous_hash = last.map(|row| row.get::<String, _>("entry_hash"));

        let mut dist_tags =
            sqlx::query_scalar::<_, String>("SELECT dist_tags FROM packages WHERE name = ?")
                .bind(&pkg.name)
                .fetch_optional(&mut *tx)
                .await?
                .map(|stored| serde_json::from_str::<HashMap<String, String>>(&stored))
                .transpose()?
                .unwrap_or_default();
        dist_tags.extend(pkg.dist_tags.clone());
        let dist_tags_json = serde_json::to_string(&dist_tags)?;
        let time_json = serde_json::to_string(&pkg.time)?;
        let maintainers_json = serde_json::to_string(&pkg.maintainers)?;
        sqlx::query(
            "INSERT INTO packages (name, description, dist_tags, maintainers, time, private) VALUES (?, ?, ?, ?, ?, ?) ON CONFLICT(name) DO UPDATE SET description = excluded.description, dist_tags = excluded.dist_tags, maintainers = excluded.maintainers, time = excluded.time, private = excluded.private, updated_at = datetime('now')",
        )
        .bind(&pkg.name)
        .bind(&pkg.description)
        .bind(&dist_tags_json)
        .bind(&maintainers_json)
        .bind(&time_json)
        .bind(pkg.private)
        .execute(&mut *tx)
        .await?;

        let mut versions = pkg.versions.iter().collect::<Vec<_>>();
        versions.sort_by(|left, right| left.0.cmp(right.0));
        for (version, package_version) in versions {
            let digest = package_version.dist.integrity.trim();
            if !crate::trusted::is_valid_sha512_integrity(digest) {
                anyhow::bail!("trusted publish requires a valid npm SHA-512 integrity digest");
            }
            let id = format!("{}@{}", pkg.name, version);
            sqlx::query(
                "INSERT INTO package_versions (id, package_name, version, data) VALUES (?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET data = excluded.data",
            )
            .bind(&id)
            .bind(&pkg.name)
            .bind(version)
            .bind(serde_json::to_string(package_version)?)
            .execute(&mut *tx)
            .await?;

            sequence = sequence
                .checked_add(1)
                .filter(|next| *next <= i64::MAX as u64)
                .ok_or_else(|| anyhow::anyhow!("trusted attestation sequence is exhausted"))?;
            let (artifact_sha256, sigstore_bundle) =
                public_attestations.get(version).ok_or_else(|| {
                    anyhow::anyhow!("trusted publish is missing its public Sigstore attestation")
                })?;
            if !crate::trusted::is_valid_sha256_hex(artifact_sha256) {
                anyhow::bail!("trusted publish requires a valid artifact SHA-256 digest");
            }
            let payload = crate::trusted::AttestationPayload {
                schema_version: 1,
                package: pkg.name.clone(),
                version: version.clone(),
                digest: digest.to_string(),
                artifact_sha256: Some(artifact_sha256.clone()),
                sequence,
                builder: crate::trusted::BuilderIdentity::from(claims),
                created_at_unix: chrono::Utc::now().timestamp().max(0) as u64,
                previous_hash: previous_hash.clone(),
            };
            let mut entry = crate::trusted::sign_attestation(payload, key)?;
            entry.sigstore_bundle = Some(sigstore_bundle.clone());
            sqlx::query(
                "INSERT INTO trusted_attestations (package, sequence, payload, entry_hash, key_id, signature, sigstore_bundle) VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&pkg.name)
            .bind(sequence as i64)
            .bind(serde_json::to_string(&entry.payload)?)
            .bind(&entry.entry_hash)
            .bind(&entry.key_id)
            .bind(&entry.signature)
            .bind(serde_json::to_string(sigstore_bundle)?)
            .execute(&mut *tx)
            .await?;
            previous_hash = Some(entry.entry_hash);
        }
        tx.commit().await?;
        Ok(())
    }
}

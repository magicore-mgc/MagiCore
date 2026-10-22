//! Owns trusted attestation status, key id, and paginated reads.
//! Sở hữu trạng thái, key id và truy vấn phân trang trusted attestation.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    pub async fn has_trusted_attestations(&self) -> Result<bool> {
        let exists: i64 = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM trusted_attestations)")
            .fetch_one(&self.db)
            .await?;
        Ok(exists != 0)
    }

    pub async fn trusted_attestation_key_id(&self) -> Result<Option<String>> {
        let rows = sqlx::query_scalar::<_, String>(
            "SELECT DISTINCT key_id FROM trusted_attestations LIMIT 2",
        )
        .fetch_all(&self.db)
        .await?;
        if rows.len() > 1 {
            anyhow::bail!("registry provenance contains records from unsupported signing keys");
        }
        Ok(rows.into_iter().next())
    }

    /// Read one immutable chain page, pinning the first page's head sequence.
    /// Đọc một trang chuỗi bất biến và giữ nguyên sequence head từ trang đầu.
    pub async fn trusted_attestations_page(
        &self,
        package: &str,
        after_sequence: u64,
        through_sequence: Option<u64>,
        limit: usize,
    ) -> Result<(u64, Vec<crate::trusted::SignedAttestation>)> {
        if limit == 0 || limit > crate::trusted::MAX_SIGNATURE_PAGE_SIZE {
            anyhow::bail!("trusted signature page size is outside the allowed range");
        }
        let current_head: i64 = sqlx::query_scalar(
            "SELECT COALESCE(MAX(sequence), 0) FROM trusted_attestations WHERE package = ?",
        )
        .bind(package)
        .fetch_one(&self.db)
        .await?;
        let current_head =
            u64::try_from(current_head).context("trusted attestation head sequence is invalid")?;
        let head_sequence = through_sequence.unwrap_or(current_head).min(current_head);
        if after_sequence >= head_sequence {
            return Ok((head_sequence, Vec::new()));
        }
        let after_sequence = i64::try_from(after_sequence)
            .context("trusted attestation cursor is outside the supported range")?;
        let head_sequence = i64::try_from(head_sequence)
            .context("trusted attestation head is outside the supported range")?;
        let rows = sqlx::query(
            "SELECT payload, entry_hash, key_id, signature, sigstore_bundle FROM trusted_attestations WHERE package = ? AND sequence > ? AND sequence <= ? ORDER BY sequence ASC LIMIT ?",
        )
        .bind(package)
        .bind(after_sequence)
        .bind(head_sequence)
        .bind(limit as i64)
        .fetch_all(&self.db)
        .await?;
        let entries = rows
            .into_iter()
            .map(|row| {
                Ok(crate::trusted::SignedAttestation {
                    payload: serde_json::from_str(&row.get::<String, _>("payload"))?,
                    entry_hash: row.get("entry_hash"),
                    key_id: row.get("key_id"),
                    signature: row.get("signature"),
                    sigstore_bundle: row
                        .get::<Option<String>, _>("sigstore_bundle")
                        .map(|value| serde_json::from_str(&value))
                        .transpose()?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok((head_sequence as u64, entries))
    }

    pub async fn trusted_attestations(
        &self,
        package: &str,
    ) -> Result<Vec<crate::trusted::SignedAttestation>> {
        let rows = sqlx::query(
            "SELECT payload, entry_hash, key_id, signature, sigstore_bundle FROM trusted_attestations WHERE package = ? ORDER BY sequence ASC",
        )
        .bind(package)
        .fetch_all(&self.db)
        .await?;
        rows.into_iter()
            .map(|row| {
                Ok(crate::trusted::SignedAttestation {
                    payload: serde_json::from_str(&row.get::<String, _>("payload"))?,
                    entry_hash: row.get("entry_hash"),
                    key_id: row.get("key_id"),
                    signature: row.get("signature"),
                    sigstore_bundle: row
                        .get::<Option<String>, _>("sigstore_bundle")
                        .map(|value| serde_json::from_str(&value))
                        .transpose()?,
                })
            })
            .collect()
    }
}

//! Validate signature key IDs, page sequence, canonical payload hash, and Ed25519 proof.
//! Kiểm tra key ID, thứ tự trang, hash payload chuẩn và chữ ký Ed25519.

use super::error::RegistryChainError;
use mgc_crypto::{Ed25519PublicKey, Ed25519Signature, ed25519_signer::verify_signature};
use sha2::{Digest, Sha256};

pub(in crate::commands::audit::signatures) fn verified_bundle_key(
    bundle: &mgc_registry_server::trusted::AttestationBundle,
) -> std::result::Result<Ed25519PublicKey, RegistryChainError> {
    let public_key = Ed25519PublicKey::from_base64(&bundle.public_key)
        .map_err(|_| RegistryChainError::Invalid)?;
    let key_hash = mgc_crypto::Blake3Hasher::hash_bytes(&public_key.0);
    let expected_key_id = hex::encode(&key_hash.0[..8]);
    if expected_key_id != bundle.key_id {
        return Err(RegistryChainError::Invalid);
    }
    Ok(public_key)
}

pub(in crate::commands::audit::signatures) fn verify_page(
    package: &str,
    protocol: &str,
    entries: &[mgc_registry_server::trusted::SignedAttestation],
    key_id: &str,
    public_key: &Ed25519PublicKey,
    first_sequence: u64,
    mut previous_hash: Option<String>,
) -> std::result::Result<Option<String>, RegistryChainError> {
    for (index, entry) in entries.iter().enumerate() {
        let sequence = first_sequence
            .checked_add(index as u64)
            .ok_or(RegistryChainError::Invalid)?;
        if entry.payload.package != package
            || entry.payload.schema_version != 1
            || entry.payload.sequence != sequence
            || entry.payload.previous_hash != previous_hash
            || entry.payload.builder.issuer.is_empty()
            || entry.payload.builder.repository.is_empty()
            || entry.payload.created_at_unix == 0
            || !entry
                .payload
                .builder
                .subject
                .starts_with(&format!("repo:{}:", entry.payload.builder.repository))
            || entry.key_id != key_id
            || match protocol {
                "npm" => {
                    !mgc_registry_server::trusted::is_valid_sha512_integrity(&entry.payload.digest)
                }
                "pypi" | "oci" => entry
                    .payload
                    .artifact_sha256
                    .as_ref()
                    .is_none_or(|digest| entry.payload.digest != format!("sha256:{digest}")),
                _ => true,
            }
        {
            return Err(RegistryChainError::Invalid);
        }
        let canonical = serde_json::to_vec(&entry.payload)?;
        let calculated_hash = hex::encode(Sha256::digest(canonical));
        if calculated_hash != entry.entry_hash {
            return Err(RegistryChainError::Invalid);
        }
        let signature = Ed25519Signature::from_base64(&entry.signature)
            .map_err(|_| RegistryChainError::Invalid)?;
        verify_signature(public_key, entry.entry_hash.as_bytes(), &signature)
            .map_err(|_| RegistryChainError::Invalid)?;
        previous_hash = Some(entry.entry_hash.clone());
    }
    Ok(previous_hash)
}

#[cfg(test)]
pub(super) fn verify_bundle(
    package: &str,
    bundle: &mgc_registry_server::trusted::AttestationBundle,
) -> std::result::Result<usize, RegistryChainError> {
    if bundle.package != package || bundle.entries.is_empty() {
        return Err(RegistryChainError::MissingOrMismatched);
    }
    if bundle.head_sequence != bundle.entries.len() as u64 {
        return Err(RegistryChainError::Invalid);
    }
    let public_key = verified_bundle_key(bundle)?;
    verify_page(
        package,
        "npm",
        &bundle.entries,
        &bundle.key_id,
        &public_key,
        1,
        None,
    )?;
    Ok(bundle.entries.len())
}

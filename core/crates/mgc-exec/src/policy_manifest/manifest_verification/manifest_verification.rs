use crate::policy_manifest::{
    ManifestError, POLICY_MANIFEST_SCHEMA, PolicyManifest, canonical_bytes, is_canonical_key_id_hex,
};

/// Verify schema, key trust, external revocation, signature and expiry.
/// Selection separately applies revocations carried by this signed snapshot.
/// Xác minh schema, trust, revocation bên ngoài, chữ ký và hạn. Bước chọn
/// manifest áp dụng riêng danh sách thu hồi có trong snapshot đã ký này.
pub fn verify_manifest(
    manifest: &PolicyManifest,
    trusted_key_ids: &[String],
    revoked_key_ids: &[String],
    now_secs: i64,
) -> Result<(), ManifestError> {
    // Unknown schema fails closed (never parse-and-hope).
    // (Schema lạ fail-closed (không parse-cầu-may).)
    if manifest.schema_version != POLICY_MANIFEST_SCHEMA {
        return Err(ManifestError::UnknownSchema(
            manifest.schema_version.clone(),
        ));
    }
    if !trusted_key_ids.contains(&manifest.key_id) {
        return Err(ManifestError::UntrustedKey);
    }
    if manifest.expires_at <= manifest.issued_at {
        return Err(ManifestError::InvalidTimeWindow);
    }
    if now_secs < manifest.issued_at {
        return Err(ManifestError::NotYetValid);
    }
    if revoked_key_ids.contains(&manifest.key_id) {
        return Err(ManifestError::RevokedKey);
    }
    let public_key = mgc_crypto::ed25519_signer::Ed25519PublicKey::from_base64(&manifest.key_id)
        .map_err(|_| ManifestError::UntrustedKey)?;
    let signature = mgc_crypto::ed25519_signer::Ed25519Signature::from_base64(&manifest.signature)
        .map_err(|_| ManifestError::BadSignature)?;
    mgc_crypto::ed25519_signer::verify_string_signature(
        &public_key,
        &canonical_bytes(manifest),
        &signature,
    )
    .map_err(|_| ManifestError::BadSignature)?;
    if now_secs >= manifest.expires_at {
        return Err(ManifestError::Expired);
    }
    if manifest
        .revoked_keys
        .iter()
        .any(|key| !is_canonical_key_id_hex(key))
    {
        return Err(ManifestError::InvalidRevocationKey);
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

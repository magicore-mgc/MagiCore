/// Revocation IDs use the same 16-character lowercase BLAKE3 hex shape as
/// `MGC_POLICY_TRUST_KEYS`.
/// (ID thu hồi dùng cùng dạng BLAKE3 hex chữ thường 16 ký tự với trust keys.)
pub(crate) fn is_canonical_key_id_hex(value: &str) -> bool {
    value.len() == 16
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// BLAKE3 key id (first 8 bytes, hex) of a base64 Ed25519 public key —
/// the same id format as `mgc.toml [trust] keys` and lockfile signers.
/// Key id BLAKE3 (8 byte đầu, hex) của khóa công khai Ed25519 base64 —
/// cùng định dạng id với `mgc.toml [trust] keys` và signer lockfile.
pub fn key_id_hex(public_key_b64: &str) -> Option<String> {
    let key = mgc_crypto::ed25519_signer::Ed25519PublicKey::from_base64(public_key_b64).ok()?;
    let digest = mgc_crypto::blake3_signer::Blake3Hasher::hash_bytes(&key.0);
    Some(digest.to_hex().chars().take(16).collect())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

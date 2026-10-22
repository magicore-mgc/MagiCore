//! Signed-message and digest functions / Hàm thông điệp ký và digest.

/// Domain-separated message prefix — Tiền tố tách miền ký.
pub const ATTESTATION_MESSAGE_DOMAIN: &str = "mgc-core-attestation/v1";
/// Canonical signed message — Thông điệp ký canonical.
pub fn canonical_message(core: &str, config_digest: &str) -> String {
    format!("{ATTESTATION_MESSAGE_DOMAIN}\n{core}\n{config_digest}")
}

/// BLAKE3 hex of bytes — BLAKE3 hex của bytes.
pub fn blake3_hex(data: &[u8]) -> String {
    let hash = mgc_crypto::blake3_signer::Blake3Hasher::hash_bytes(data);
    hex::encode(hash.0)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

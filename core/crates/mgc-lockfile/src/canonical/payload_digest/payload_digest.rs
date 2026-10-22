//! Hashes only the canonical signed payload bytes.
//! Chỉ băm byte payload canonical được ký.

use crate::canonical::{LockfilePayload, canonical_toml};
/// Digest of the canonical payload (`blake3-<base64>`) — Digest của
/// payload canonical.
pub fn payload_digest(payload: &LockfilePayload) -> String {
    use mgc_crypto::blake3_signer::Blake3Hasher;
    let bytes = canonical_toml(payload);
    let hash = Blake3Hasher::hash_bytes(bytes.as_bytes());
    format!("blake3-{}", hash.to_base64())
}

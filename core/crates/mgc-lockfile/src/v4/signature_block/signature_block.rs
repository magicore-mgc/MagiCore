//! Represent the inline lockfile signature record.
//! Biểu diễn record chữ ký nhúng trong lockfile.

use serde::{Deserialize, Serialize};

/// Inline signature block (single-file lock, no sidecar) — Khối chữ ký
/// nhúng (lock một file, không sidecar).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureBlock {
    /// Signature algorithm (`ed25519`) — Thuật toán chữ ký.
    pub algorithm: String,
    /// Key id (first 8 bytes of BLAKE3(pubkey), hex) — Id khóa.
    pub key_id: String,
    /// Base64 public key (offline verification without a keyring).
    pub public_key: String,
    /// Digest of the canonical payload (`blake3-<base64>`) — Digest của
    /// payload canonical.
    pub digest: String,
    /// Signing timestamp (out-of-payload) — Thời điểm ký.
    pub signed_at: String,
    /// The signature (`ed25519-<base64>`) — Chữ ký.
    pub signature: String,
}

#[cfg(test)]
#[path = "test/signature_block.rs"]
mod tests;

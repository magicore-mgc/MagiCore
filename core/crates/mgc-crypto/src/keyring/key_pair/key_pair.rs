//! Ed25519 key-pair representation and generation — Biểu diễn và tạo key pair Ed25519.
//! Preserve public fields and signer conversion — Giữ field public và chuyển đổi signer.

use crate::ed25519_signer::{Ed25519PublicKey, Ed25519Signer};
use crate::keyring::error::KeyringError;
use crate::{CryptoError, CryptoResult};
use ring::rand::SystemRandom;
use ring::signature::Ed25519KeyPair;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Key pair wrapper — Key pair wrapper
#[derive(Clone, Serialize, Deserialize)]
pub struct KeyPair {
    /// PKCS8-encoded private key — Khóa riêng encode PKCS8
    pub private_key_pkcs8: Vec<u8>,
    /// Public key — Khóa công khai
    pub public_key: Ed25519PublicKey,
    /// Key ID (fingerprint) — Key ID (fingerprint)
    pub key_id: String,
    /// Creation timestamp — Timestamp tạo
    pub created_at: u64,
}

impl fmt::Debug for KeyPair {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("KeyPair")
            .field("private_key_pkcs8", &"[REDACTED]")
            .field("public_key", &self.public_key)
            .field("key_id", &self.key_id)
            .field("created_at", &self.created_at)
            .finish()
    }
}

impl KeyPair {
    /// Generate new key pair — Tạo key pair mới
    pub fn generate() -> CryptoResult<Self> {
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|e| CryptoError::KeyringFailed(format!("key generation failed: {:?}", e)))?;

        let signer = Ed25519Signer::from_pkcs8(pkcs8_bytes.as_ref())?;
        let public_key = signer.public_key();

        // Generate key ID from public key hash — Tạo key ID từ hash khóa công khai
        let key_id = Self::compute_key_id(&public_key);

        let created_at = unix_timestamp(std::time::SystemTime::now())?;

        Ok(KeyPair {
            private_key_pkcs8: pkcs8_bytes.as_ref().to_vec(),
            public_key,
            key_id,
            created_at,
        })
    }

    /// Compute key ID from public key (first 8 bytes of BLAKE3 hash)
    /// Tính key ID từ khóa công khai (8 bytes đầu của BLAKE3 hash)
    fn compute_key_id(public_key: &Ed25519PublicKey) -> String {
        use crate::blake3_signer::Blake3Hasher;
        let hash = Blake3Hasher::hash_bytes(&public_key.0);
        hex::encode(&hash.0[..8])
    }

    /// Get signer from this key pair — Lấy signer từ key pair này
    pub fn signer(&self) -> CryptoResult<Ed25519Signer> {
        Ed25519Signer::from_pkcs8(&self.private_key_pkcs8)
    }
}

/// Convert a system time to seconds since the Unix epoch without panicking.
/// Đổi system time thành giây từ Unix epoch và trả lỗi nếu đồng hồ đi lùi.
pub(super) fn unix_timestamp(time: std::time::SystemTime) -> CryptoResult<u64> {
    time.duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|_| KeyringError::SystemTimeBeforeUnixEpoch.into())
}
// Hex encoding helper — Helper encode hex
mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

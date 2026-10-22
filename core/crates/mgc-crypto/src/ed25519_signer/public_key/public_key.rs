//! Ed25519 public-key value — Value public key Ed25519.
//! Keep the historical serde shape and Base64 width check — Giữ dạng serde và kiểm tra độ dài Base64 lịch sử.

use serde::{Deserialize, Serialize};

use crate::{CryptoError, CryptoResult, ed25519_signer::encoding::base64};

/// Ed25519 public key bytes — Byte khóa công khai Ed25519.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ed25519PublicKey(pub Vec<u8>);

impl Ed25519PublicKey {
    /// Encode the key bytes as Base64 STANDARD — Encode byte key thành Base64 STANDARD.
    pub fn to_base64(&self) -> String {
        base64::encode(&self.0)
    }

    /// Parse a 32-byte public key from Base64 STANDARD — Parse public key 32 byte từ Base64 STANDARD.
    pub fn from_base64(value: &str) -> CryptoResult<Self> {
        let bytes = base64::decode(value).map_err(CryptoError::from)?;
        if bytes.len() != 32 {
            return Err(
                crate::ed25519_signer::error::Ed25519Error::InvalidPublicKeyLength(bytes.len())
                    .into(),
            );
        }
        Ok(Self(bytes))
    }
}

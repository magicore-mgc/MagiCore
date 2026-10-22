//! Ed25519 key-pair signing owner — Owner ký bằng key pair Ed25519.
//! Keep key creation and signing delegated to ring — Giữ tạo key và ký do ring thực hiện.

use ring::signature::{Ed25519KeyPair, KeyPair as _};

use super::{Ed25519PublicKey, Ed25519Signature};
use crate::{CryptoResult, ed25519_signer::error::Ed25519Error};

/// Ed25519 signer backed by ring — Signer Ed25519 được ring cung cấp.
pub struct Ed25519Signer {
    key_pair: Ed25519KeyPair,
}

impl Ed25519Signer {
    /// Create a signer from a 32-byte seed — Tạo signer từ seed 32 byte.
    pub fn from_seed(seed: &[u8; 32]) -> CryptoResult<Self> {
        let key_pair = Ed25519KeyPair::from_seed_unchecked(seed)
            .map_err(|error| Ed25519Error::SeedRejected(format!("{error:?}")))?;
        Ok(Self { key_pair })
    }

    /// Create a signer from PKCS#8 bytes — Tạo signer từ byte PKCS#8.
    pub fn from_pkcs8(pkcs8_bytes: &[u8]) -> CryptoResult<Self> {
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes)
            .map_err(|error| Ed25519Error::Pkcs8Rejected(format!("{error:?}")))?;
        Ok(Self { key_pair })
    }

    /// Return the signer's public key — Trả về public key của signer.
    pub fn public_key(&self) -> Ed25519PublicKey {
        Ed25519PublicKey(self.key_pair.public_key().as_ref().to_vec())
    }

    /// Sign a byte message — Ký message dạng byte.
    pub fn sign(&self, message: &[u8]) -> Ed25519Signature {
        let signature = self.key_pair.sign(message);
        Ed25519Signature(signature.as_ref().to_vec())
    }

    /// Sign a UTF-8 string — Ký chuỗi UTF-8.
    pub fn sign_string(&self, value: &str) -> Ed25519Signature {
        self.sign(value.as_bytes())
    }
}

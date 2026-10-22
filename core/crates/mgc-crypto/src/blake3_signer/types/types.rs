//! Fixed-width digest value and stable text representations.
//! Giá trị digest độ dài cố định và biểu diễn văn bản ổn định.
//! Giá trị digest độ dài cố định và biểu diễn văn bản.

use crate::blake3_signer::encoding::{base64, hex};
use crate::blake3_signer::error::EncodingError;
use crate::{CryptoError, CryptoResult};

/// A 256-bit BLAKE3 digest.
/// Digest BLAKE3 có độ dài 256 bit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blake3Hash(pub [u8; 32]);

impl Blake3Hash {
    /// Encode as lowercase hexadecimal.
    /// Mã hóa thành chuỗi hex chữ thường.
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }

    /// Parse a 64-character hexadecimal digest.
    /// Đọc digest hex có độ dài 64 ký tự.
    pub fn from_hex(value: &str) -> CryptoResult<Self> {
        let bytes = hex::decode(value)
            .map_err(|error| CryptoError::Blake3Failed(format!("invalid hex: {error}")))?;
        let digest: [u8; 32] = bytes.try_into().map_err(|bytes: Vec<u8>| {
            CryptoError::Blake3Failed(EncodingError::InvalidDigestLength(bytes.len()).to_string())
        })?;
        Ok(Self(digest))
    }

    /// Encode as standard base64.
    /// Mã hóa theo chuẩn base64.
    pub fn to_base64(&self) -> String {
        base64::encode(&self.0)
    }

    /// Parse a standard base64 digest.
    /// Đọc digest theo chuẩn base64.
    pub fn from_base64(value: &str) -> CryptoResult<Self> {
        let bytes = base64::decode(value)
            .map_err(|error| CryptoError::Blake3Failed(format!("base64 decode: {error}")))?;
        let digest: [u8; 32] = bytes.try_into().map_err(|bytes: Vec<u8>| {
            CryptoError::Blake3Failed(EncodingError::InvalidDigestLength(bytes.len()).to_string())
        })?;
        Ok(Self(digest))
    }
}

impl std::fmt::Display for Blake3Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex())
    }
}

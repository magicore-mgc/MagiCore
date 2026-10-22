//! SRI value, parser and display representation — Value SRI, parser và biểu diễn Display.
//! Preserve the current split grammar and public field shape — Giữ grammar split và dạng field public hiện tại.

use crate::{CryptoResult, blake3_signer::Blake3Hash};

/// Subresource Integrity value — Giá trị Subresource Integrity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SriHash {
    /// Algorithm identifier — Định danh thuật toán.
    pub algorithm: String,
    /// Encoded hash value — Giá trị hash đã encode.
    pub hash: String,
}

impl SriHash {
    /// Parse an SRI value by splitting on one hyphen — Parse SRI bằng cách tách tại một dấu gạch nối.
    pub fn parse(s: &str) -> CryptoResult<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(
                crate::integrity::error::IntegrityError::InvalidSriFormat(s.to_string()).into(),
            );
        }
        Ok(Self {
            algorithm: parts[0].to_string(),
            hash: parts[1].to_string(),
        })
    }

    /// Create a BLAKE3 SRI value — Tạo value SRI BLAKE3.
    pub fn from_blake3(hash: &Blake3Hash) -> Self {
        Self {
            algorithm: "blake3".to_string(),
            hash: hash.to_base64(),
        }
    }
}

impl std::fmt::Display for SriHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.algorithm, self.hash)
    }
}

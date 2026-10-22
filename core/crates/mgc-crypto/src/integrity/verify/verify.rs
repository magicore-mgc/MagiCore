//! In-memory SRI verification — Xác minh SRI trong bộ nhớ.
//! Preserve algorithm dispatch, diagnostics and BLAKE3 comparison — Giữ dispatch, diagnostic và phép so sánh BLAKE3.

use super::super::{error::IntegrityError, integrity::IntegrityVerifier, sri_hash::SriHash};
use crate::CryptoResult;
use crate::blake3_signer::{Blake3Hash, Blake3Hasher};

impl IntegrityVerifier {
    /// Verify data against an SRI value — Xác minh dữ liệu với value SRI.
    pub fn verify(data: &[u8], sri: &SriHash) -> CryptoResult<()> {
        match sri.algorithm.as_str() {
            "blake3" => {
                let actual = Blake3Hasher::hash_bytes(data);
                let expected = Blake3Hash::from_base64(&sri.hash).map_err(IntegrityError::from)?;
                if actual == expected {
                    Ok(())
                } else {
                    Err(IntegrityError::HashMismatch {
                        expected: expected.to_string(),
                        actual: actual.to_string(),
                    }
                    .into())
                }
            }
            "sha256" | "sha512" => {
                Err(IntegrityError::UnsupportedAlgorithm(sri.algorithm.clone()).into())
            }
            _ => Err(IntegrityError::UnknownAlgorithm(sri.algorithm.clone()).into()),
        }
    }
}

//! SRI file verification and computation — Verify và tính SRI cho file.
//! Preserve full-file reads for verification and streaming for computation — Giữ đọc toàn file khi verify và streaming khi compute.

use super::super::{error::IntegrityError, integrity::IntegrityVerifier, sri_hash::SriHash};
use crate::CryptoResult;
use crate::blake3_signer::Blake3Hasher;

impl IntegrityVerifier {
    /// Verify the complete file contents against SRI — Verify toàn bộ nội dung file với SRI.
    pub fn verify_file(path: &std::path::Path, sri: &SriHash) -> CryptoResult<()> {
        let data = std::fs::read(path).map_err(IntegrityError::from)?;
        Self::verify(&data, sri)
    }

    /// Compute a BLAKE3 SRI value by streaming the file — Tính SRI BLAKE3 bằng streaming file.
    pub fn compute_file(path: &std::path::Path) -> CryptoResult<SriHash> {
        let hash = Blake3Hasher::hash_file(path).map_err(IntegrityError::from)?;
        Ok(SriHash::from_blake3(&hash))
    }
}

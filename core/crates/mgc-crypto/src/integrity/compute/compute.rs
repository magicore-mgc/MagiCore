//! In-memory SRI computation — Tính SRI trong bộ nhớ.
//! Keep BLAKE3 as the only computed integrity algorithm — Giữ BLAKE3 là thuật toán integrity duy nhất được tính.

use super::super::{integrity::IntegrityVerifier, sri_hash::SriHash};
use crate::blake3_signer::Blake3Hasher;

impl IntegrityVerifier {
    /// Compute a BLAKE3 SRI value for data — Tính value SRI BLAKE3 cho dữ liệu.
    pub fn compute(data: &[u8]) -> SriHash {
        let hash = Blake3Hasher::hash_bytes(data);
        SriHash::from_blake3(&hash)
    }
}

//! Compatibility namespace for the former memory-mapped hashing method.
//! Namespace tương thích cho method hash memory-map trước đây.
//! Namespace tương thích cho method hash memory-map cũ.

pub(in crate::blake3_signer) fn hash_file_mmap(
    path: &std::path::Path,
) -> crate::CryptoResult<crate::blake3_signer::types::Blake3Hash> {
    crate::blake3_signer::file::stream::hash_file(path)
}

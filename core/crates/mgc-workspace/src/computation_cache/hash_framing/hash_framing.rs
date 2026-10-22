// Length-prefixed hash fields — trường hash có tiền tố độ dài

pub(in crate::computation_cache) fn update_hash_part(hasher: &mut sha2::Sha256, bytes: &[u8]) {
    use sha2::Digest;

    // Prefix every field with its byte length to preserve unambiguous boundaries.
    // Thêm độ dài byte trước mỗi trường để ranh giới dữ liệu luôn rõ ràng.
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

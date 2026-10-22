// Dependency composite fingerprint — fingerprint tổng hợp dependency

use super::super::hash_framing::update_hash_part;
use std::collections::BTreeMap;

/// Compute a composite hash from the source hash and dependency hashes.
/// Tính composite hash từ source_hash và hash của các dependency.
pub fn compute_composite_hash(source_hash: &str, dep_hashes: &BTreeMap<String, String>) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"mgc-package-composite-hash-v2\0");
    update_hash_part(&mut hasher, source_hash.as_bytes());
    hasher.update((dep_hashes.len() as u64).to_be_bytes());
    for (dep_name, dep_hash) in dep_hashes {
        update_hash_part(&mut hasher, dep_name.as_bytes());
        update_hash_part(&mut hasher, dep_hash.as_bytes());
    }
    hex::encode(hasher.finalize())
}

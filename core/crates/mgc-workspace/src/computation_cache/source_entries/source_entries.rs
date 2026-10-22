// Source-entry digest — digest các mục source

use super::super::hash_framing::update_hash_part;
use std::collections::BTreeMap;

pub(in crate::computation_cache) fn hash_source_entries(
    file_hashes: &BTreeMap<String, String>,
) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"mgc-package-source-hash-v2\0");
    hasher.update((file_hashes.len() as u64).to_be_bytes());
    for (rel_path, f_hash) in file_hashes {
        update_hash_part(&mut hasher, rel_path.as_bytes());
        update_hash_part(&mut hasher, f_hash.as_bytes());
    }
    hex::encode(hasher.finalize())
}

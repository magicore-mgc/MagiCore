//! Canonical tree-hash framing contracts.
//! Contract framing tree-hash canonical.

use sha2::{Digest, Sha256};

use super::super::update_tree_hasher;

#[test]
fn tree_signature_changes_with_path_size_and_executable_state() {
    fn signature(path: &str, size: u64, executable: bool) -> String {
        let mut hasher = Sha256::new();
        update_tree_hasher(&mut hasher, path, size, executable, "content-digest");
        hex::encode(hasher.finalize())
    }

    let baseline = signature("index.js", 3, false);
    assert_ne!(baseline, signature("other.js", 3, false));
    assert_ne!(baseline, signature("index.js", 4, false));
    assert_ne!(baseline, signature("index.js", 3, true));
}

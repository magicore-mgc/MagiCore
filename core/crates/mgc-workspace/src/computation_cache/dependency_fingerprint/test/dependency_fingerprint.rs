// dependency_fingerprint tests — test cho dependency_fingerprint

use super::compute_composite_hash;
use std::collections::BTreeMap;

#[test]
fn composite_hash_frames_dependency_fields() {
    let source_hash = "a".repeat(64);
    let mut split_dependencies = BTreeMap::new();
    split_dependencies.insert("a".to_string(), "b".repeat(64));
    split_dependencies.insert("c".to_string(), "d".repeat(64));

    let mut combined_dependency = BTreeMap::new();
    combined_dependency.insert(format!("a{}c", "b".repeat(64)), "d".repeat(64));

    assert_ne!(
        compute_composite_hash(&source_hash, &split_dependencies),
        compute_composite_hash(&source_hash, &combined_dependency),
        "different dependency boundaries must produce different composite hashes"
    );
}

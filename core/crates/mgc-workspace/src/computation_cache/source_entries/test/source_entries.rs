// source_entries tests — test cho source_entries

use super::hash_source_entries;
use std::collections::BTreeMap;

#[test]
fn source_hash_frames_path_and_content_fields() {
    let mut split_entries = BTreeMap::new();
    split_entries.insert("61".to_string(), "b".repeat(64));
    split_entries.insert("62".to_string(), "c".repeat(64));

    let mut combined_entry = BTreeMap::new();
    combined_entry.insert(format!("61{}62", "b".repeat(64)), "c".repeat(64));

    assert_ne!(
        hash_source_entries(&split_entries),
        hash_source_entries(&combined_entry),
        "different path/content boundaries must produce different source hashes"
    );
}

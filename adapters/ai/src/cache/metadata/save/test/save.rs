//! Metadata-save cases — kiểm tra public write/read roundtrip.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::save::save_metadata;
use crate::cache::metadata::{CacheEntry, load_metadata, metadata_path};
use std::time::SystemTime;
use tempfile::TempDir;

#[test]
fn saves_and_loads_metadata_without_changing_public_fields() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("model.bin");
    std::fs::write(&model, b"weights").unwrap();
    let entry = CacheEntry {
        model_id: "bert/base".into(),
        size_bytes: 7,
        cached_at: SystemTime::now(),
        last_accessed: SystemTime::now(),
        checksum: None,
    };

    save_metadata(&model, &entry).unwrap();
    let loaded = load_metadata(&model).unwrap();

    assert_eq!(loaded.model_id, entry.model_id);
    assert!(metadata_path(&model).is_file());
}

#[test]
fn serialized_field_names_match_the_existing_json_contract() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("model.bin");
    std::fs::write(&model, b"weights").unwrap();
    let entry = CacheEntry {
        model_id: "bert/base".into(),
        size_bytes: 7,
        cached_at: SystemTime::now(),
        last_accessed: SystemTime::now(),
        checksum: None,
    };

    save_metadata(&model, &entry).unwrap();

    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(metadata_path(&model)).unwrap()).unwrap();
    let object = json.as_object().unwrap();
    for name in [
        "model_id",
        "size_bytes",
        "cached_at",
        "last_accessed",
        "checksum",
    ] {
        assert!(object.contains_key(name), "missing {name}");
    }
}

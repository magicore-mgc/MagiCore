//! Cache-entry cases — kiểm tra shape và snapshot từ path.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::cache_entry::CacheEntry;
use std::time::SystemTime;
use tempfile::TempDir;

#[test]
fn from_path_keeps_model_id_and_regular_file_size() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("model.bin");
    std::fs::write(&model, vec![0u8; 1024]).unwrap();

    let entry = CacheEntry::from_path("test/model", &model).unwrap();

    assert_eq!(entry.model_id, "test/model");
    assert_eq!(entry.size_bytes, 1024);
}

#[test]
fn public_snapshot_fields_remain_constructible() {
    let now = SystemTime::now();
    let entry = CacheEntry {
        model_id: "model".into(),
        size_bytes: 4,
        cached_at: now,
        last_accessed: now,
        checksum: None,
    };

    assert_eq!(entry.size_bytes, 4);
}

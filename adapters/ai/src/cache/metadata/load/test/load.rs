//! Metadata-load cases — kiểm tra fallback, JSON lỗi và no-follow.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::load::load_metadata;
use crate::cache::metadata::{CacheEntry, metadata_path};
use std::time::SystemTime;
use tempfile::TempDir;

#[test]
fn missing_sidecar_uses_path_metadata_fallback() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("model.bin");
    std::fs::write(&model, b"weights").unwrap();

    let entry = load_metadata(&model).unwrap();

    assert_eq!(entry.model_id, "unknown");
    assert_eq!(entry.size_bytes, 7);
}

#[test]
fn valid_sidecar_roundtrips_existing_schema() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("model.bin");
    std::fs::write(&model, b"weights").unwrap();
    let entry = CacheEntry {
        model_id: "org/model".into(),
        size_bytes: 7,
        cached_at: SystemTime::now(),
        last_accessed: SystemTime::now(),
        checksum: Some("abc".into()),
    };
    std::fs::write(metadata_path(&model), serde_json::to_vec(&entry).unwrap()).unwrap();

    let loaded = load_metadata(&model).unwrap();

    assert_eq!(loaded.model_id, "org/model");
    assert_eq!(loaded.size_bytes, 7);
    assert_eq!(loaded.checksum.as_deref(), Some("abc"));
}

#[test]
fn malformed_sidecar_returns_error() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("model.bin");
    std::fs::write(&model, b"weights").unwrap();
    std::fs::write(metadata_path(&model), b"{").unwrap();

    assert!(load_metadata(&model).is_err());
}

#[test]
fn directory_sidecar_is_rejected() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("model.bin");
    std::fs::write(&model, b"weights").unwrap();
    std::fs::create_dir(metadata_path(&model)).unwrap();

    assert!(load_metadata(&model).is_err());
}

#[cfg(unix)]
#[test]
fn refuses_sidecar_symlink_without_reading_target() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let model = temp.path().join("model.bin");
    let target = temp.path().join("outside.json");
    std::fs::write(&model, b"weights").unwrap();
    std::fs::write(&target, b"{}").unwrap();
    symlink(&target, metadata_path(&model)).unwrap();

    assert!(load_metadata(&model).is_err());
    assert_eq!(std::fs::read(&target).unwrap(), b"{}");
}

//! Prune integration cases — kiểm tra API public phối hợp các owner cache.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::*;
use crate::cache::metadata::{CacheEntry, save_metadata};
use crate::registry::Registry;
use std::time::{Duration, SystemTime};
use tempfile::TempDir;

fn local_registry(path: &std::path::Path) -> Registry {
    Registry::Local(path.to_path_buf())
}

fn save_entry(path: &std::path::Path, model_id: &str, age_days: u64, idle_days: u64) {
    let now = SystemTime::now();
    let metadata = CacheEntry {
        model_id: model_id.into(),
        size_bytes: 1,
        cached_at: now - Duration::from_secs(age_days * 86_400),
        last_accessed: now - Duration::from_secs(idle_days * 86_400),
        checksum: None,
    };
    save_metadata(path, &metadata).unwrap();
}

#[test]
fn older_than_removes_model_and_its_metadata_sidecar() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("old.bin");
    std::fs::write(&model, [0u8; 100]).unwrap();
    save_entry(&model, "old", 10, 0);
    let sidecar = model.with_extension("meta.json");
    let expected_bytes = 100 + std::fs::metadata(&sidecar).unwrap().len();

    let result = prune_cache(&local_registry(temp.path()), PruneStrategy::OlderThan(5)).unwrap();

    assert_eq!(result.removed_count, 1);
    assert_eq!(result.bytes_freed, expected_bytes);
    assert!(!model.exists());
    assert!(!sidecar.exists());
}

#[test]
fn keep_recent_removes_all_but_newest_accessed_entry() {
    let temp = TempDir::new().unwrap();
    let old = temp.path().join("old.bin");
    let recent = temp.path().join("recent.bin");
    std::fs::write(&old, [0u8; 4]).unwrap();
    std::fs::write(&recent, [0u8; 6]).unwrap();
    save_entry(&old, "old", 1, 12);
    save_entry(&recent, "recent", 1, 1);

    let result = prune_cache(&local_registry(temp.path()), PruneStrategy::KeepRecent(1)).unwrap();

    assert_eq!(result.removed_count, 1);
    assert!(!old.exists());
    assert!(recent.exists());
}

#[test]
fn keep_recent_retains_an_entry_with_corrupt_metadata() {
    let temp = TempDir::new().unwrap();
    let unknown = temp.path().join("unknown.bin");
    std::fs::write(&unknown, [0u8; 4]).unwrap();
    std::fs::write(unknown.with_extension("meta.json"), b"not-json").unwrap();

    let result = prune_cache(&local_registry(temp.path()), PruneStrategy::KeepRecent(0)).unwrap();

    assert_eq!(result.removed_count, 0);
    assert!(unknown.exists());
}

#[test]
fn larger_than_uses_actual_file_bytes_instead_of_stale_metadata() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("large.bin");
    std::fs::write(&model, [0u8; 50]).unwrap();
    save_entry(&model, "large", 0, 0);

    let result = prune_cache(&local_registry(temp.path()), PruneStrategy::LargerThan(20)).unwrap();

    assert_eq!(result.removed_count, 1);
    assert!(!model.exists());
}

#[test]
fn remove_model_rejects_parent_path_before_mutation() {
    let temp = TempDir::new().unwrap();
    let cache = temp.path().join("cache");
    std::fs::create_dir(&cache).unwrap();
    let sentinel = temp.path().join("outside-marker");
    std::fs::write(&sentinel, b"keep").unwrap();

    assert!(remove_model(&local_registry(&cache), "..").is_err());
    assert_eq!(std::fs::read(&sentinel).unwrap(), b"keep");
    assert!(cache.exists());
}

#[test]
fn remove_model_returns_zero_for_missing_entry() {
    let temp = TempDir::new().unwrap();

    let bytes = remove_model(&local_registry(temp.path()), "missing").unwrap();

    assert_eq!(bytes, 0);
}

#[test]
fn remove_model_removes_file_and_sidecar_and_reports_logical_bytes() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("model--v1");
    let sidecar = model.with_extension("meta.json");
    std::fs::write(&model, [0u8; 12]).unwrap();
    std::fs::write(&sidecar, b"metadata").unwrap();

    let bytes = remove_model(&local_registry(temp.path()), "model/v1").unwrap();

    assert_eq!(bytes, 12 + b"metadata".len() as u64);
    assert!(!model.exists());
    assert!(!sidecar.exists());
}

#[test]
fn remove_model_removes_nested_directory_and_counts_regular_files() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("bert--base");
    let snapshot = model.join("snapshots").join("rev-1");
    std::fs::create_dir_all(&snapshot).unwrap();
    std::fs::write(snapshot.join("weights.bin"), [0u8; 200]).unwrap();

    let bytes = remove_model(&local_registry(temp.path()), "bert/base").unwrap();

    assert_eq!(bytes, 200);
    assert!(!model.exists());
}

#[test]
fn prune_keeps_a_sidecar_that_was_ambiguous_at_discovery_time() {
    let temp = TempDir::new().unwrap();
    let first = temp.path().join("weights");
    let second = temp.path().join("weights.bin");
    let shared_sidecar = temp.path().join("weights.meta.json");
    std::fs::write(&first, [0u8; 3]).unwrap();
    std::fs::write(&second, [0u8; 5]).unwrap();
    std::fs::write(&shared_sidecar, b"shared metadata").unwrap();

    let result = prune_cache(&local_registry(temp.path()), PruneStrategy::LargerThan(0)).unwrap();

    assert_eq!(result.removed_count, 2);
    assert!(!first.exists());
    assert!(!second.exists());
    assert!(shared_sidecar.exists());
}

#[test]
fn unused_for_removes_entry_according_to_last_access_metadata() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("idle.bin");
    std::fs::write(&model, [0u8; 4]).unwrap();
    save_entry(&model, "idle", 1, 10);

    let result = prune_cache(&local_registry(temp.path()), PruneStrategy::UnusedFor(5)).unwrap();

    assert_eq!(result.removed_count, 1);
    assert!(!model.exists());
}

//! Logical-size cases — kiểm tra đệ quy file thường và bỏ link vòng.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::super::logical_size::logical_size;
use tempfile::TempDir;

#[test]
fn measures_regular_files_recursively() {
    let temp = TempDir::new().unwrap();
    let nested = temp.path().join("snapshot").join("weights");
    std::fs::create_dir_all(&nested).unwrap();
    std::fs::write(temp.path().join("index"), [1u8; 3]).unwrap();
    std::fs::write(nested.join("model.bin"), [1u8; 11]).unwrap();

    assert_eq!(logical_size(temp.path()).unwrap(), 14);
}

#[test]
fn measures_deep_directories_without_recursive_call_stack() {
    let temp = TempDir::new().unwrap();
    let mut nested = temp.path().to_path_buf();
    for _ in 0..96 {
        nested.push("d");
        std::fs::create_dir(&nested).unwrap();
    }
    std::fs::write(nested.join("weights.bin"), [1u8; 7]).unwrap();

    assert_eq!(logical_size(temp.path()).unwrap(), 7);
}

#[cfg(unix)]
#[test]
fn skips_external_and_cyclic_symlinks() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let cache = temp.path().join("cache");
    let external = temp.path().join("external.bin");
    std::fs::create_dir(&cache).unwrap();
    std::fs::write(&external, [1u8; 17]).unwrap();
    std::fs::write(cache.join("inside.bin"), [1u8; 5]).unwrap();
    symlink(&external, cache.join("external-link")).unwrap();
    symlink(&cache, cache.join("cycle-link")).unwrap();

    assert_eq!(logical_size(&cache).unwrap(), 5);
}

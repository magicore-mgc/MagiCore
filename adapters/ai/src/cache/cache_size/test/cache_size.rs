//! Cache-size cases — kiểm tra số byte, symlink và cache root sai dạng.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use crate::cache::cache_size::cache_size;
use crate::registry::Registry;
use tempfile::TempDir;

#[test]
fn counts_regular_content_recursively() {
    let temp = TempDir::new().unwrap();
    let nested = temp.path().join("model").join("snapshot");
    std::fs::create_dir_all(&nested).unwrap();
    std::fs::write(nested.join("weights.bin"), [0u8; 300]).unwrap();

    assert_eq!(
        cache_size(&Registry::Local(temp.path().to_path_buf())).unwrap(),
        300
    );
}

#[test]
fn missing_cache_root_has_zero_size() {
    let temp = TempDir::new().unwrap();
    let missing = temp.path().join("missing");

    assert_eq!(cache_size(&Registry::Local(missing)).unwrap(), 0);
}

#[test]
fn non_directory_cache_root_returns_an_error() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("cache-file");
    std::fs::write(&root, b"data").unwrap();

    assert!(cache_size(&Registry::Local(root)).is_err());
}

#[cfg(unix)]
#[test]
fn skips_external_symlink_targets() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let cache = temp.path().join("cache");
    let outside = temp.path().join("outside.bin");
    std::fs::create_dir(&cache).unwrap();
    std::fs::write(cache.join("inside.bin"), [0u8; 4]).unwrap();
    std::fs::write(&outside, [0u8; 31]).unwrap();
    symlink(outside, cache.join("linked.bin")).unwrap();

    assert_eq!(cache_size(&Registry::Local(cache)).unwrap(), 4);
}

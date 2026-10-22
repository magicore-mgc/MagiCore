//! Cache-presence cases — kiểm tra API true/false theo filesystem.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::is_cached::is_cached;
use crate::registry::Registry;
use tempfile::TempDir;

#[test]
fn missing_model_path_is_not_cached() {
    let temp = TempDir::new().unwrap();

    assert!(!is_cached(&Registry::Local(temp.path().to_path_buf()), "missing-model").unwrap());
}

#[test]
fn existing_flattened_model_path_is_cached() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("bert--base");
    std::fs::create_dir(&model).unwrap();

    assert!(is_cached(&Registry::Local(temp.path().to_path_buf()), "bert/base").unwrap());
}

#[cfg(unix)]
#[test]
fn follows_an_existing_model_symlink_like_the_previous_api() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let model = temp.path().join("model");
    let alias = temp.path().join("bert--base");
    std::fs::create_dir(&model).unwrap();
    symlink(&model, &alias).unwrap();

    assert!(is_cached(&Registry::Local(temp.path().to_path_buf()), "bert/base").unwrap());
}

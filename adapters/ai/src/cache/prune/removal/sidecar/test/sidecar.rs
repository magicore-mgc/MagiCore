//! Sidecar behavior cases — kiểm tra association, byte count và symlink boundary.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::sidecar::find_unique_sidecar;
use crate::cache::prune::removal::removal::remove_regular_sidecar;
use tempfile::TempDir;

#[test]
fn resolves_sidecar_only_for_a_unique_model_path() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("weights.bin");
    std::fs::write(&model, b"model").unwrap();
    let expected = model.with_extension("meta.json");

    assert_eq!(find_unique_sidecar(&model).unwrap(), Some(expected.clone()));

    std::fs::write(temp.path().join("weights"), b"other model").unwrap();
    assert_eq!(find_unique_sidecar(&model).unwrap(), None);
}

#[test]
fn removes_regular_sidecar_and_reports_its_logical_bytes() {
    let temp = TempDir::new().unwrap();
    let sidecar = temp.path().join("model.meta.json");
    std::fs::write(&sidecar, b"metadata").unwrap();

    assert_eq!(remove_regular_sidecar(&sidecar).unwrap(), 8);
    assert!(!sidecar.exists());
}

#[cfg(unix)]
#[test]
fn does_not_remove_a_sidecar_symlink_or_its_target() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let target = temp.path().join("external.json");
    let sidecar = temp.path().join("model.meta.json");
    std::fs::write(&target, b"outside").unwrap();
    symlink(&target, &sidecar).unwrap();

    assert_eq!(remove_regular_sidecar(&sidecar).unwrap(), 0);
    assert!(target.exists());
    assert!(sidecar.exists());
}

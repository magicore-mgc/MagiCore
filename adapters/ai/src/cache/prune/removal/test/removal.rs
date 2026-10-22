//! Cache removal cases — kiểm tra xóa entry, sidecar và ranh giới symlink.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::super::removal::{remove_candidate, remove_model_path};
use super::super::{candidate_discovery::discover_candidates, error::PruneError};
use crate::registry::Registry;
use tempfile::TempDir;

#[test]
fn removes_entry_and_regular_sidecar_and_counts_both() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("model.bin");
    let sidecar = temp.path().join("model.meta.json");
    std::fs::write(&model, [1u8; 7]).unwrap();
    std::fs::write(&sidecar, [1u8; 3]).unwrap();
    let candidates = discover_candidates(
        &Registry::Local(temp.path().to_path_buf()),
        temp.path(),
        true,
    )
    .unwrap();

    assert_eq!(remove_candidate(&candidates[0]).unwrap(), Some(10));
    assert!(!model.exists());
    assert!(!sidecar.exists());
}

#[cfg(unix)]
#[test]
fn refuses_symlink_model_path_without_removing_target() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let target = temp.path().join("outside.bin");
    let link = temp.path().join("model");
    std::fs::write(&target, [1u8; 9]).unwrap();
    symlink(&target, &link).unwrap();

    assert!(matches!(
        remove_model_path(&link),
        Err(PruneError::SymbolicLinkEntry)
    ));
    assert!(target.exists());
    assert!(link.exists());
}

#[test]
fn keeps_a_sidecar_when_another_model_path_shares_it() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("weights");
    let other_model = temp.path().join("weights.bin");
    let shared_sidecar = temp.path().join("weights.meta.json");
    std::fs::write(&model, [1u8; 3]).unwrap();
    std::fs::write(&other_model, [1u8; 5]).unwrap();
    std::fs::write(&shared_sidecar, b"{}").unwrap();

    assert_eq!(remove_model_path(&model).unwrap(), Some(3));
    assert!(!model.exists());
    assert!(other_model.exists());
    assert!(shared_sidecar.exists());
}

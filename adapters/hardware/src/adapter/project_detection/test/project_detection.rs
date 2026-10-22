#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Detect a project from the shared MagiCore manifest marker.
//! Nhận diện project bằng marker manifest MagiCore dùng chung.

use crate::adapter::HardwareAdapter;
use mgc_types::capabilities::ProjectDetector;
use tempfile::tempdir;

#[test]
fn detector_accepts_any_mgc_manifest_and_rejects_plain_directories() {
    let project = tempdir().unwrap();
    assert!(!HardwareAdapter.can_handle(project.path()));
    std::fs::write(project.path().join("mgc.toml"), "").unwrap();
    assert!(HardwareAdapter.can_handle(project.path()));
}

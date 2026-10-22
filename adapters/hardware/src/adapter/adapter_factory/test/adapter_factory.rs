#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Select the adapter only when the shared MagiCore manifest exists.
//! Chỉ chọn adapter khi manifest MagiCore dùng chung tồn tại.

use crate::adapter::adapter_for;
use tempfile::tempdir;

#[test]
fn factory_requires_the_mgc_manifest_marker() {
    let project = tempdir().unwrap();
    assert!(adapter_for(project.path()).is_none());
    std::fs::write(project.path().join("mgc.toml"), "").unwrap();
    assert!(adapter_for(project.path()).is_some());
}

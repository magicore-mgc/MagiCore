#![cfg(test)]

//! Keep the public adapter and factory routes available.
//! Giữ route adapter và factory công khai.

use crate::adapter::{CicdAdapter, adapter_for};
use tempfile::tempdir;

#[test]
fn stable_cicd_adapter_exports_remain_available() {
    let project = tempdir().expect("temporary project directory should be created");
    let _ = CicdAdapter::CAPABILITIES;
    assert!(adapter_for(project.path()).is_none());
}

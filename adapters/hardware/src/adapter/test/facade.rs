#![cfg(test)]

//! Keep the public hardware adapter routes wired through the facade.
//! Giữ các route adapter hardware công khai qua facade.

use crate::adapter::{HardwareAdapter, adapter_for};
use tempfile::tempdir;

#[test]
fn stable_hardware_adapter_exports_remain_available() {
    let _ = HardwareAdapter::CAPABILITIES;
    let project = tempdir().expect("temporary project directory should be created");
    assert!(adapter_for(project.path()).is_none());
}

#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Verify project metadata readers use the configured IoT fields.
//! Xác minh reader đọc đúng field IoT đã cấu hình.

use crate::framework::manifest_metadata::{manifest_is_iot, target_from_manifest};
use tempfile::TempDir;

#[test]
fn manifest_identity_and_target_are_read_from_mgc_toml() {
    let temp = TempDir::new().unwrap();
    std::fs::write(
        temp.path().join("mgc.toml"),
        "ecosystem = 'iot'\n[iot]\ntarget = 'riscv32imc-unknown-none-elf'\n",
    )
    .unwrap();
    assert!(manifest_is_iot(temp.path()));
    assert_eq!(
        target_from_manifest(temp.path()).as_deref(),
        Some("riscv32imc-unknown-none-elf")
    );
}

#[test]
fn non_iot_project_without_iot_markers_is_not_identified() {
    let temp = TempDir::new().unwrap();
    std::fs::write(temp.path().join("mgc.toml"), "ecosystem = 'web'\n").unwrap();
    assert!(!manifest_is_iot(temp.path()));
    assert_eq!(target_from_manifest(temp.path()), None);
}

#[test]
fn iot_table_alone_identifies_the_project() {
    let temp = TempDir::new().unwrap();
    std::fs::write(
        temp.path().join("mgc.toml"),
        "[iot]\nframework = 'platformio'\n",
    )
    .unwrap();
    assert!(manifest_is_iot(temp.path()));
}

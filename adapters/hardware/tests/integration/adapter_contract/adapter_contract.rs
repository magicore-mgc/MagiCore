//! Adapter Contract behavior tests.
//! Checks Hardware adapter identity and project-detection contract.

use crate::integration::support::tmp;
use mgc_hardware_adapter::HardwareAdapter;
use mgc_types::capabilities::{CoreIdent, ProjectDetector};

pub fn adapter_name_and_ecosystem() {
    assert_eq!(HardwareAdapter.name(), "hardware");
    assert_eq!(format!("{:?}", HardwareAdapter.ecosystem()), "Hardware");
}

pub fn can_handle_returns_true_for_hardware_ecosystem() {
    let dir = tmp("ch-true");
    std::fs::write(dir.join("mgc.toml"), "ecosystem = \"hardware\"\n").unwrap();
    assert!(HardwareAdapter.can_handle(&dir));
}

pub fn can_handle_returns_false_for_plain_dir() {
    let dir = tmp("ch-false");
    assert!(!HardwareAdapter.can_handle(&dir));
}

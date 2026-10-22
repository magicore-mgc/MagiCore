#![cfg(test)]

//! Keep hardware's registry identity stable.
//! Giữ định danh hardware ổn định trong registry.

use crate::adapter::HardwareAdapter;
use mgc_types::capabilities::CoreIdent;

#[test]
fn identity_uses_hardware_id_name_and_ecosystem() {
    assert_eq!(HardwareAdapter.core_id(), "hardware");
    assert_eq!(HardwareAdapter.name(), "hardware");
    assert_eq!(format!("{:?}", HardwareAdapter.ecosystem()), "Hardware");
}

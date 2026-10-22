#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! Check Cargo-compatible package-name character boundaries.
//! Kiểm tra ranh giới ký tự tên package tương thích Cargo.

use super::super::validate_esp32_cargo_package_name;

#[test]
fn accepts_ascii_and_unicode_cargo_names() {
    for name in [
        "device",
        "device_2",
        "device-board",
        "_device",
        "café-开发_2",
    ] {
        assert!(validate_esp32_cargo_package_name(name).is_ok(), "{name}");
    }
}

#[test]
fn rejects_empty_digit_started_and_non_identifier_names() {
    for name in [
        "",
        "2device",
        "device name",
        "device.name",
        "device::name",
        "device/path",
    ] {
        assert!(
            matches!(
                validate_esp32_cargo_package_name(name),
                Err(mgc_types::MgError::InvalidPackageName(_))
            ),
            "{name:?}"
        );
    }
}

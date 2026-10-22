#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Verify IoT marker priority and configured framework selection.
//! Xác minh ưu tiên marker IoT và framework trong config.

use crate::framework::{IotFramework, detect_framework};
use tempfile::TempDir;

#[test]
fn marker_detection_keeps_existing_priority() {
    let temp = TempDir::new().unwrap();
    std::fs::write(temp.path().join("west.yml"), "manifest: {}\n").unwrap();
    std::fs::write(temp.path().join("Cargo.toml"), "[package]\nname='x'\n").unwrap();
    assert_eq!(detect_framework(temp.path()), Some(IotFramework::Zephyr));

    std::fs::write(temp.path().join("platformio.ini"), "[env:test]\n").unwrap();
    assert_eq!(
        detect_framework(temp.path()),
        Some(IotFramework::Platformio)
    );
}

#[test]
fn configured_framework_wins_and_other_ecosystem_stops_fallback() {
    let temp = TempDir::new().unwrap();
    std::fs::write(
        temp.path().join("mgc.toml"),
        "ecosystem = 'iot'\n[iot]\nframework = 'zephyr-arm'\n",
    )
    .unwrap();
    std::fs::write(temp.path().join("platformio.ini"), "[env:test]\n").unwrap();
    assert_eq!(detect_framework(temp.path()), Some(IotFramework::Zephyr));

    std::fs::write(
        temp.path().join("mgc.toml"),
        "ecosystem = 'web'\n[iot]\nframework = 'zephyr'\n",
    )
    .unwrap();
    assert_eq!(detect_framework(temp.path()), None);
}

#[test]
fn cargo_manifest_is_the_last_iot_marker_fallback() {
    let temp = TempDir::new().unwrap();
    std::fs::write(temp.path().join("Cargo.toml"), "[package]\nname='x'\n").unwrap();
    assert_eq!(detect_framework(temp.path()), Some(IotFramework::Esp32Rust));
}

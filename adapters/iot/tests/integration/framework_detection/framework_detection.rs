//! Framework Detection behavior tests.
//! Test hành vi cho nhóm framework detection.

use crate::integration::support::tmp;
use mgc_iot_adapter::{IotFramework, detect_framework};

pub fn detect_platformio_via_ini() {
    let dir = tmp("pio");
    std::fs::write(
        dir.join("platformio.ini"),
        "[env:esp32dev]\nplatform = espressif32\n",
    )
    .unwrap();
    assert_eq!(detect_framework(&dir), Some(IotFramework::Platformio));
}

pub fn detect_zephyr_via_west_yml() {
    let dir = tmp("zephyr");
    std::fs::write(dir.join("west.yml"), "manifest:\n  projects: []\n").unwrap();
    assert_eq!(detect_framework(&dir), Some(IotFramework::Zephyr));
}

pub fn detect_returns_none_for_empty_dir() {
    let dir = tmp("empty");
    assert!(detect_framework(&dir).is_none());
}

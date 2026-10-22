//! Adapter Discovery behavior tests.
//! Test hành vi cho nhóm adapter discovery.

use crate::integration::support::tmp;
use mgc_iot_adapter::adapter_for;

pub fn adapter_for_returns_some_for_platformio() {
    let dir = tmp("af-pio");
    std::fs::write(dir.join("platformio.ini"), "[env:esp32]\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    assert_eq!(a.framework(), "platformio");
}

pub fn adapter_for_returns_none_for_plain_dir() {
    let dir = tmp("af-none");
    assert!(adapter_for(&dir).is_none());
}

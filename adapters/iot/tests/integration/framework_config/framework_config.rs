//! Framework Config behavior tests.
//! Test hành vi cho nhóm framework config.

use crate::integration::support::tmp;
use mgc_iot_adapter::{IotFramework, detect_framework};

pub fn detect_esp32_rust_via_mgc_toml() {
    let dir = tmp("esp32-rust");
    std::fs::write(
        dir.join("mgc.toml"),
        "ecosystem = \"iot\"\n\n[iot]\nframework = \"esp32-rust\"\n",
    )
    .unwrap();
    assert_eq!(detect_framework(&dir), Some(IotFramework::Esp32Rust));
}

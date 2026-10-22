//! Stable IoT processor API and output dispatch.
//! API IotProcessor ổn định và điều phối output.

use std::path::Path;

use anyhow::Result;

#[path = "catalog/mod.rs"]
mod catalog;
#[path = "esp32_rust/mod.rs"]
mod esp32_rust;
#[path = "generic_rust/mod.rs"]
mod generic_rust;
#[path = "platformio/mod.rs"]
mod platformio;
#[path = "zephyr/mod.rs"]
mod zephyr;

pub struct IotProcessor;

impl IotProcessor {
    /// Frameworks this processor generates directly without a registry layer.
    /// Framework được processor tự sinh trực tiếp, không cần registry layer.
    pub fn supports(framework: &str) -> bool {
        catalog::supports(framework)
    }

    /// Map a published board identifier to its owning generator.
    /// Ánh xạ ID board được công bố về generator sở hữu.
    pub fn framework_for_board(board: &str) -> Option<&'static str> {
        catalog::framework_for_board(board)
    }

    /// Select the registered default board for a framework.
    /// Chọn board mặc định đã đăng ký cho framework.
    pub fn default_board(framework: &str) -> Option<&'static str> {
        catalog::default_board(framework)
    }

    pub fn files(target: &Path, name: &str, framework: &str, board: &str) -> Result<()> {
        match framework {
            "platformio" | "firmware" => platformio::write(target, board),
            "zephyr" | "zephyr-arm" => zephyr::write(target),
            "esp32-rust" => esp32_rust::write(target, name, board),
            _ => generic_rust::write(target, name),
        }
    }
}

//! IoT scaffold: platformio/zephyr/esp32-rust templates.

use std::path::Path;

use anyhow::Result;

use super::{slugify, write_file};

pub struct IotProcessor;

impl IotProcessor {
    /// Frameworks this processor generates directly (no registry layer
    /// needed). Anything else keeps the layer-required error instead of
    /// receiving a mislabeled fallback (e.g. board variant `esp32dev`
    /// must NOT get a generic Cargo project).
    /// (Framework processor tự sinh — ngoài danh sách vẫn fail rõ.)
    pub fn supports(framework: &str) -> bool {
        matches!(
            framework,
            "platformio"
                | "firmware"
                | "zephyr"
                | "zephyr-arm"
                | "esp32-rust"
                | "esp32"
                | "esp32c3"
                | "esp32s3"
                | "esp32dev"
                | "nodemcu-32s"
                | "nrf52dk_nrf52832"
                | "stm32f4_disc"
        )
    }

    /// Map a published board identifier to the generator that owns it.
    /// Ánh xạ ID board được công bố về generator tương ứng.
    pub fn framework_for_board(board: &str) -> Option<&'static str> {
        match board {
            "esp32" | "esp32c3" | "esp32s3" => Some("esp32-rust"),
            "esp32dev" | "nodemcu-32s" => Some("platformio"),
            "nrf52dk_nrf52832" | "stm32f4_disc" => Some("zephyr-arm"),
            _ => None,
        }
    }

    /// Select the registered default board for a framework.
    /// Chọn board mặc định đã đăng ký cho framework.
    pub fn default_board(framework: &str) -> Option<&'static str> {
        match framework {
            "esp32-rust" => Some("esp32c3"),
            "platformio" | "firmware" => Some("esp32dev"),
            "zephyr" | "zephyr-arm" => Some("nrf52dk_nrf52832"),
            _ => None,
        }
    }

    pub fn files(target: &Path, name: &str, framework: &str, board: &str) -> Result<()> {
        match framework {
            "platformio" | "firmware" => {
                write_file(
                    &target.join("platformio.ini"),
                    &format!(
                        "[env:{board}]\nplatform = espressif32\nboard = {board}\nframework = arduino\n"
                    ),
                )?;
                write_file(
                    &target.join("src").join("main.cpp"),
                    "#include <Arduino.h>\n\nvoid setup() {\n}\n\nvoid loop() {\n}\n",
                )?;
            }
            "zephyr" | "zephyr-arm" => {
                write_file(
                    &target.join("west.yml"),
                    "manifest:\n  version: 0.13\n  projects: []\n",
                )?;
                write_file(
                    &target.join("CMakeLists.txt"),
                    "cmake_minimum_required(VERSION 3.20.0)\nfind_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})\nproject(magicore_firmware)\ntarget_sources(app PRIVATE src/main.c)\n",
                )?;
                write_file(
                    &target.join("prj.conf"),
                    "CONFIG_PRINTK=y\nCONFIG_MAIN_STACK_SIZE=2048\n",
                )?;
                write_file(
                    &target.join("src").join("main.c"),
                    "#include <zephyr/kernel.h>\n#include <zephyr/sys/printk.h>\n\nint main(void)\n{\n    printk(\"MagiCore Zephyr firmware scaffold\\n\");\n    return 0;\n}\n",
                )?;
            }
            "esp32-rust" => {
                // esp-hal on crates.io (verified resolvable: esp-hal 1.2.2
                // exposes matching esp32/esp32c3/esp32s3 features. Building
                // needs the Espressif toolchain (not provisioned here).
                // (esp-hal 1.2.2 có feature khớp ba board; build cần toolchain Espressif.)
                let feature = match board {
                    "esp32" => "esp32",
                    "esp32c3" => "esp32c3",
                    "esp32s3" => "esp32s3",
                    _ => anyhow::bail!("No verified esp-hal feature for board '{board}'"),
                };
                write_file(
                    &target.join("Cargo.toml"),
                    &format!(
                        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nesp-hal = {{ version = \"1.2.2\", features = [\"{feature}\"] }}\n",
                        slugify(name),
                    ),
                )?;
                write_file(
                    &target.join("src").join("main.rs"),
                    "#![no_std]\n#![no_main]\n\nuse esp_hal::main;\n\n#[main]\nfn main() -> ! {\n    loop {}\n}\n",
                )?;
            }
            _ => {
                write_file(
                    &target.join("Cargo.toml"),
                    &format!(
                        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
                        slugify(name)
                    ),
                )?;
                write_file(
                    &target.join("src").join("main.rs"),
                    "#![no_std]\n#![no_main]\n\n#[no_mangle]\npub extern \"C\" fn main() -> ! {\n    loop {}\n}\n",
                )?;
            }
        }

        Ok(())
    }
}

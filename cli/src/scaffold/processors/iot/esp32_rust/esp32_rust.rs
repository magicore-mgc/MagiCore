//! ESP-HAL Rust writer implementation.
//! Triển khai writer rust esp-hal.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

#[path = "board_feature/mod.rs"]
mod board_feature;
#[path = "error/mod.rs"]
mod error;

pub(in crate::scaffold::processors::iot::implementation) fn write(
    target: &Path,
    name: &str,
    board: &str,
) -> Result<()> {
    // esp-hal on crates.io (verified resolvable: esp-hal 1.2.2
    // exposes matching esp32/esp32c3/esp32s3 features. Building
    // needs the Espressif toolchain (not provisioned here).
    // (esp-hal 1.2.2 có feature khớp ba board; build cần toolchain Espressif.)
    let feature = board_feature::for_board(board)?;
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
    Ok(())
}

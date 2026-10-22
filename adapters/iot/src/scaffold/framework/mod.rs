//! Framework-specific IoT project file generators.
//! Các generator file project IoT theo từng framework.

mod esp32_rust;
mod platformio;
mod toml_string;
mod zephyr;

pub(super) use esp32_rust::{scaffold_esp32, validate_esp32_cargo_package_name};
pub(super) use platformio::scaffold_platformio;
pub(super) use toml_string::serialize_toml_string;
pub(super) use zephyr::scaffold_zephyr;

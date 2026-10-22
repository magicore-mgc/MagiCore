//! ESP32 Rust starter-file owner.
//! Owner file khởi tạo ESP32 Rust.

mod cargo_package_name;
mod scaffold_esp32;
pub(in crate::scaffold) use cargo_package_name::validate_esp32_cargo_package_name;
pub(in crate::scaffold) use scaffold_esp32::scaffold_esp32;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

//! Wire ESP32 Rust Cargo package-name validation and its tests.
//! Nối xác thực tên package Cargo cho ESP32 Rust và test của owner.

mod cargo_package_name;
pub(in crate::scaffold) use cargo_package_name::validate_esp32_cargo_package_name;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

//! Rust Cargo manifest parser facade.
//! Facade bộ phân tích manifest Cargo của Rust.

mod cargo_manifest;

pub(super) use cargo_manifest::read_cargo_manifest;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

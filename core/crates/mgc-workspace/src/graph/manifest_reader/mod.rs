//! Polyglot workspace manifest reader facade.
//! Facade bộ đọc manifest workspace đa ngôn ngữ.

mod cargo_manifest;
mod error;
mod manifest_reader;
mod package_json;
mod pyproject;

pub use manifest_reader::read_package_manifest;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

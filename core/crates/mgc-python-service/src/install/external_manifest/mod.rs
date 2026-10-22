//! Detect Python dependency files outside the native PEP 621 lane.
//! Nhận diện file dependency Python ngoài lane PEP 621 native.

mod error;
mod external_manifest;

pub(super) use external_manifest::external_python_manifest_name;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

//! Wires static Package.swift source reading and validation.
//! Nối thao tác đọc và kiểm tra source Package.swift tĩnh.

mod package_source;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub use self::package_source::scan_package_swift_text;
pub(super) use self::package_source::{
    manifest_path, read_literal_dependencies, unsupported_static_swift_manifest,
};

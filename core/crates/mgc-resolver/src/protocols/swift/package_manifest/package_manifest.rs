//! Backward-compatible path for the former combined parser module.
//! Đường dẫn tương thích ngược cho module parser gộp trước đây.
pub use super::{parse_dump_package, parse_swift_package_deps};

#[cfg(test)]
#[path = "package_manifest/test/mod.rs"]
mod test;

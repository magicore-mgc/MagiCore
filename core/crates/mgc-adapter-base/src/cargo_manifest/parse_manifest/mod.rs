//! Parse the Cargo dependency subset supported by the native resolver.
//! Parse tập dependency Cargo được native resolver hỗ trợ.

mod parse_manifest;

pub(super) use self::parse_manifest::parse_manifest;

#[cfg(test)]
#[path = "test/parse_manifest.rs"]
mod tests;

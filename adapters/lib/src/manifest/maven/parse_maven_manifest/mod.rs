//! Wiring and exports / Khai báo và xuất module.
mod parse_maven_manifest;
pub(crate) use self::parse_maven_manifest::parse_maven_manifest;

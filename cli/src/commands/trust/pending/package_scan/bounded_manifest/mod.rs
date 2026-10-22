//! Bounded manifest-reader owner wiring.
//! Wiring owner đọc manifest có giới hạn.

mod bounded_manifest;
pub(super) use self::bounded_manifest::read_bounded_package_json;

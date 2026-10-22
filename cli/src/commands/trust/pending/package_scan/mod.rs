//! Package-scan wiring and exported scan operation.
//! Wiring và export thao tác quét package.

mod bounded_manifest;
mod package_json;
mod package_scan;

pub(super) use self::package_scan::scan_scripted_packages;

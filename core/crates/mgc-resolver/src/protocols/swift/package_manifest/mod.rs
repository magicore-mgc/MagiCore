//! Swift package manifest parser ownership and stable exports.
//! Phân quyền parser manifest Swift và các export ổn định.
mod dump_package;
pub mod package_manifest;
mod source_manifest;

pub use dump_package::parse_dump_package;
pub use source_manifest::parse_swift_package_deps;
pub(super) use source_manifest::{balanced_close, extract_requirement, quoted_after};

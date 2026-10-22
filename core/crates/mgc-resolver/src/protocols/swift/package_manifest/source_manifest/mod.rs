//! Package.swift text parser exports.
//! Export parser văn bản Package.swift.
mod source_manifest;
pub use source_manifest::parse_swift_package_deps;
pub(in crate::protocols::swift) use source_manifest::{
    balanced_close, extract_requirement, quoted_after,
};

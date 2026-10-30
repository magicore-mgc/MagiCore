mod error;
mod materialize;
#[path = "source_readiness/mod.rs"]
mod source_readiness;

pub(super) use error::MaterializeResult;
pub(super) use materialize::materialize;
pub use source_readiness::NON_ARCHIVE_SOURCE_FILES;
pub use source_readiness::{
    CARGO_SOURCE_MARKER, has_cargo_source_marker, registry_package_dir, registry_source_root,
    write_cargo_source_marker,
};

#[cfg(test)]
mod source_readiness_test;

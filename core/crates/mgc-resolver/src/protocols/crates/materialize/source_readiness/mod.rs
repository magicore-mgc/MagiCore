//! Cargo source readiness marker owner wiring.
//! Wiring owner marker sẵn sàng của source Cargo.

#[path = "source_readiness.rs"]
mod implementation;

pub use implementation::NON_ARCHIVE_SOURCE_FILES;
pub use implementation::{
    CARGO_SOURCE_MARKER, has_cargo_source_marker, registry_package_dir, registry_source_root,
    write_cargo_source_marker,
};

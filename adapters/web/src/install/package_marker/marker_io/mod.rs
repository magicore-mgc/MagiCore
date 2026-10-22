//! Marker path, JSON persistence, and marker-level predicates.
//! Path marker, lưu JSON và predicate cấp marker.

mod marker_io;

pub use marker_io::{
    extracted_marker_has_content_signature, extracted_marker_matches_fast,
    extracted_package_marker_path, read_extracted_package_marker, write_extracted_package_marker,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

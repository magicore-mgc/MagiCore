//! Extracted-package cache marker owners and stable public routes.
//! Owner marker cache gói đã extract và các route public ổn định.

mod archive_signature;
mod digest;
mod executable;
mod extracted_content;
mod marker_io;
mod materialized_package;
mod package_marker;
mod tree_digest;

pub use archive_signature::{
    common_tarball_root_prefix, path_to_signature_string, sanitize_tarball_signature_path,
    tarball_content_signature, tarball_content_signature_from_path,
    tarball_content_signature_from_reader,
};
pub(crate) use digest::SHA256_READ_BUFFER_BYTES;
pub use digest::{
    compute_sha256_hex, compute_sha256_hex_from_path, compute_sha256_hex_from_reader,
};
pub(crate) use executable::{metadata_is_executable, mode_is_executable};
pub use extracted_content::extracted_content_matches;
pub use marker_io::{
    extracted_marker_has_content_signature, extracted_marker_matches_fast,
    extracted_package_marker_path, read_extracted_package_marker, write_extracted_package_marker,
};
pub use materialized_package::{materialized_package_matches, write_materialized_package_marker};
pub use package_marker::{
    expected_extracted_package_marker_fast, expected_extracted_package_marker_fast_from_path,
    expected_extracted_package_marker_fast_from_reader,
    expected_extracted_package_marker_from_bytes, expected_extracted_package_marker_from_path,
    expected_extracted_package_marker_from_reader, local_extracted_package_root,
};
pub(crate) use tree_digest::update_tree_hasher;

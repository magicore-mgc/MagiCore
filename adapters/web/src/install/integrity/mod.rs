//! Web tarball integrity API. / API integrity tarball Web.

mod integrity;

pub use integrity::{
    compute_sha256_b64_str, prepare_verified_tarball_for_cache, verify_sri_integrity,
    verify_tarball_file_integrity, verify_tarball_integrity, verify_tarball_reader_integrity,
};

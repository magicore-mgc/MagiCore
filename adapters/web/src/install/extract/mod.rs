//! Web package extraction facade and module wiring.
//! Facade extraction package Web và wiring module.
mod archive_verify;
mod cached_files;
mod claims;
mod locks;
mod materialize;
mod package_root;
mod staging;
mod warm_cache;

pub use crate::install::package_marker::*;
pub use archive_verify::import_and_verify_cached_root_from_tarball;
#[cfg(windows)]
pub(crate) use cached_files::windows_opened_path_is_within;
#[cfg(test)]
pub(crate) use cached_files::{
    cached_archive_file_matches, cached_file_identity, cached_package_files,
};
pub use claims::{CasClaimContext, cas_claim_context, claim_ctx};
pub use locks::{extracted_package_root_lock, tarball_prefetch_lock};
pub use materialize::ensure_extracted_package_root_with_marker;
pub use package_root::{
    ensure_extracted_package_root, ensure_extracted_package_root_from_bytes,
    ensure_extracted_package_root_from_file, locate_package_dir,
};

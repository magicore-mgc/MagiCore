//! Stable tar extraction API and focused implementation owners.
//! API giải nén tar ổn định và các owner triển khai chuyên trách.

mod archive_limits;
mod archive_path;
mod cas;
mod cas_publish;
mod cas_spool;
mod cas_tree;
mod error;
mod extract_target;
mod profile;
mod tarball_file;
mod tarball_reader;
#[cfg(test)]
mod test_support;

use self::error::{ExtractionError, ExtractionResult};

pub use self::cas::{
    extract_tarball_to_cas_and_link, extract_tarball_to_cas_and_verify_existing_root,
    extract_tarball_to_rebuildable_cas_and_link,
    extract_tarball_to_rebuildable_cas_and_verify_existing_root,
};
pub use self::tarball_file::extract_tarball;
pub use self::tarball_reader::extract_tarball_from_reader;

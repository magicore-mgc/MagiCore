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
mod install_hook_guard;
mod profile;
mod tarball_file;
mod tarball_reader;
pub use self::install_hook_guard::ALLOW_INSTALL_HOOKS_ENV;
#[cfg(test)]
#[path = "test/support/mod.rs"]
mod test_support;

use self::error::{ExtractionError, ExtractionResult};

pub use self::cas::{
    extract_tarball_to_cas_and_link, extract_tarball_to_cas_and_verify_existing_root,
    extract_tarball_to_rebuildable_cas_and_link,
    extract_tarball_to_rebuildable_cas_and_verify_existing_root,
};
pub use self::install_hook_guard::InstallHookGuard;
pub use self::tarball_file::extract_tarball;
pub use self::tarball_reader::extract_tarball_from_reader;

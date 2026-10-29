//! Shared safe tar.gz extraction and Cargo archive verification.
//! Giải nén tar.gz an toàn dùng chung và xác minh archive Cargo.

mod archive;
mod archive_limits;
mod archive_path;
mod cargo_source_hashes;
mod crate_extraction;
mod error;
mod extract_root;
mod extract_target;
mod extracted_file_write;
pub mod install_hook;
#[cfg(test)]
pub(crate) mod tar_extraction;
#[cfg(not(test))]
mod tar_extraction;
use self::error::{ArchiveError, ArchiveResult};
#[cfg(test)]
#[path = "test/support/mod.rs"]
pub(crate) mod test_support;

pub use self::archive::{cargo_source_file_hashes, extract_crate_tar_gz, extract_tar_gz};
pub(crate) use self::archive::{
    ensure_extract_file_target, ensure_extract_root, write_extracted_file,
};
pub use self::install_hook::{
    ALLOW_INSTALL_HOOKS_ENV, InstallHook, InstallHookKind, InstallHookPolicy, classify_entry,
    hooks_allowed_for, reject_install_hooks, warn_hook_escape_hatch,
};

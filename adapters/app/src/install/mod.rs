//! App adapter install facade and module wiring.
//! Facade install App và wiring các module.
mod atomic_write_project_file;
mod content_store;
pub mod fetch;
mod flutter_native;
mod flutter_package_config;
mod install;
mod lockfile;
mod package_entry;
mod pub_archive_cache;
mod swift_native;
pub mod verify;

pub(crate) use atomic_write_project_file::atomic_write_project_file;
pub(crate) use content_store::content_store;
pub(crate) use flutter_native::install_flutter_native;
#[cfg(test)]
pub(crate) use flutter_native::preflight_flutter_archives;
#[cfg(test)]
use flutter_package_config::{
    flutter_project_root_uri, write_flutter_package_config,
    write_flutter_package_config_with_sdk_root,
};
pub use install::run_install;
pub(crate) use install::run_install_with_swift_protocol;
pub(crate) use lockfile::{direct_package_ids, write_canonical_lock_with_roots};
pub(crate) use package_entry::entry_from_package;

pub(crate) use swift_native::install_swift_native;

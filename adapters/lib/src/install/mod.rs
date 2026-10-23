//! Lib install module wiring and compatibility exports.
//! Wiring module install Lib và export tương thích.

pub mod cargo_compat;
mod cargo_lock;
pub mod fetch;
// Public for integration tests proving cross-project store reuse; semver remains crate-internal.
// Public cho integration test chứng minh dùng lại store giữa project; semver vẫn nội bộ crate.
pub mod shared_store;
pub mod verify;

mod go_native_install;
mod install;
mod lock_coverage;
mod lock_read;
mod lock_write;
mod maven_native_install;
mod native_install_context;
mod nuget_native_install;
mod python_native_install;
mod python_runtime;
mod rust_native_install;

pub(super) use cargo_lock::{atomic_write_cargo_lock, prepare_empty_cargo_lock};
pub(crate) use install::run_install;
pub(super) use lock_coverage::{
    ecosystem_for_language, lock_entry_for, markers_for, validate_lock_coverage,
};
pub(super) use lock_read::validate_existing_lock_for_install;
pub(crate) use lock_read::{
    complete_lock_packages_from_existing, needs_existing_lock_rows, read_existing_lock,
};
pub(super) use lock_write::{direct_package_ids, write_canonical_lock_with_roots};
pub(super) use native_install_context::content_store;
pub use native_install_context::entry_from_package;
pub(crate) use python_runtime::native_python_installed_packages;
pub(super) use python_runtime::python_project_owner;
pub use python_runtime::{native_python_path_entries, native_python_path_entries_from_store};
pub(super) use rust_native_install::read_cached_rust_artifact;

#[cfg(test)]
pub(super) use lock_write::write_canonical_lock;
#[cfg(test)]
pub(super) use python_native_install::{
    read_cached_python_artifact, validate_python_importable_graph,
};
#[cfg(test)]
pub(super) use python_runtime::{python_site_dirname, select_python_packages_for_owner};
#[cfg(test)]
pub(super) use rust_native_install::{install_rust_cached, preflight_rust_artifact_cache};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

//! Lockfile and dependency lifecycle diagnostics.
//! Lỗi lockfile và vòng đời dependency.

mod error;

pub use error::{
    add_version_conflict, add_version_unsupported, cargo_toml_no_deps, frozen_lock_mismatch,
    frozen_lock_missing, install_app_packages_use_add, install_lib_packages_use_add,
    invalid_dep_id, lock_missing_install, lockfile_newer, outdated_no_registry_response,
    stale_lock_needs_install, update_unknown_package, why_package_not_in_lock,
    why_requires_lockfile_v2,
};

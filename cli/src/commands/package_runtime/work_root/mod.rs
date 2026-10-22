//! Work Root owner wiring.
//! Nối owner work_root.

mod error;
mod work_root;
pub use error::registry_package_cache_path_unsafe;
pub(crate) use work_root::PackageRunPurpose;
pub(super) use work_root::work_root_at;

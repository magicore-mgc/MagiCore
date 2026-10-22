//! Run Registry Package owner wiring.
//! Nối owner run_registry_package.

mod error;
mod run_registry_package;
pub use error::registry_package_execution_cwd_not_real;
pub(crate) use run_registry_package::run_registry_package_in;

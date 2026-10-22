//! Install Policy owner wiring.
//! Nối owner install_policy.

mod error;
mod install_policy;
pub use error::registry_package_invalid_caller_policy;
pub(super) use install_policy::{PACKAGE_RUNNER_PROJECT_NAME, PACKAGE_RUNNER_PROJECT_VERSION};
pub(crate) use install_policy::{registry_install_args, registry_project_config};

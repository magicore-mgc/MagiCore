//! Stable shared package-runtime module wiring.
//! Nối module runtime package dùng chung, giữ API caller hiện tại.

mod bin_selection;
mod execute_package;
mod install_policy;
mod installed_binary;
mod installed_metadata;
mod native_install;
mod node_runtime;
mod package_runtime;
mod request_validation;
mod run_registry_package;
mod runner_workspace;
mod work_root;

pub use bin_selection::{
    registry_package_ambiguous_bin, registry_package_invalid_bin, registry_package_undeclared_bin,
};
pub use install_policy::registry_package_invalid_caller_policy;
pub use installed_binary::{
    registry_package_bin_target_mismatch, registry_package_binary_outside_project,
    registry_package_no_binary,
};
pub use installed_metadata::registry_package_invalid_bins;
pub use node_runtime::registry_package_missing_host_node;
pub(crate) use package_runtime::*;
pub use request_validation::registry_package_invalid_request;
pub use run_registry_package::registry_package_execution_cwd_not_real;
pub use work_root::registry_package_cache_path_unsafe;

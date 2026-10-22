//! CLI diagnostic facade — preserves the existing public factory paths.
//! Facade diagnostic CLI — giữ nguyên các đường dẫn factory public hiện có.

mod audit;
mod automation;
mod build;
mod config;
mod dependency;
mod dependency_gate;
mod deploy;
mod dev;
mod hardware;
mod install;
mod lockfile;
mod migration;
mod model;
mod optimizer;
mod policy;
mod project;
mod registry;
mod runtime;
mod scaffold;
mod self_update;
mod store;
mod template;
mod test_quality;
mod trust;
mod web;
mod web_scaffold;

pub use audit::*;

pub use automation::*;

pub use build::*;

pub use config::*;

pub use dependency::*;

pub use dependency_gate::*;

pub use deploy::*;

pub use dev::*;

pub use hardware::*;

pub use install::*;

pub use lockfile::*;

pub use migration::*;

pub use model::*;

pub use optimizer::*;

pub use crate::commands::package_runtime::{
    registry_package_ambiguous_bin, registry_package_bin_target_mismatch,
    registry_package_binary_outside_project, registry_package_cache_path_unsafe,
    registry_package_execution_cwd_not_real, registry_package_invalid_bin,
    registry_package_invalid_bins, registry_package_invalid_caller_policy,
    registry_package_invalid_request, registry_package_missing_host_node,
    registry_package_no_binary, registry_package_undeclared_bin,
};

pub use policy::*;

pub use project::*;

pub use registry::*;

pub use runtime::*;

pub use scaffold::*;

pub use self_update::*;

pub use store::*;

pub use template::*;
pub(crate) use template::{TemplateHttpStatus, TemplateTransportUnavailable};

pub use test_quality::*;

pub use trust::*;

pub use web::*;

pub use web_scaffold::*;

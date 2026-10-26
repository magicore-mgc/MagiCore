//! Web dev entry, shared target types and workspace target aggregation.
//! Entry Web dev, type target dùng chung và gom target workspace.
use super::*;

mod dev;
mod dev_optimizer;
mod endpoint_args;
mod native_launch;
mod package_launch;
mod runtime_policy;
mod script_lookup;
mod target_collection;
mod types;

use local_binary::prepend_path;

pub use dev::dev_at_root;

pub(in crate::commands::core::web::implementation) use endpoint_args::append_dev_endpoint_args;
#[cfg(test)]
pub(in crate::commands::core::web::implementation) use native_launch::cargo_run_args_from_managed_source;
pub(in crate::commands::core::web::implementation) use native_launch::infer_native_dev_launch;
pub(in crate::commands::core::web::implementation) use package_launch::build_dev_launch;
pub(in crate::commands::core::web::implementation) use runtime_policy::{
    detect_runtime_from_tokens, validate_runtime_args,
};
pub(in crate::commands::core::web::implementation) use script_lookup::{
    read_dev_script, read_script,
};
pub(in crate::commands::core::web::implementation) use types::{DevLaunch, DevTarget};

#[path = "local_binary/mod.rs"]
mod local_binary;
#[path = "native_environment/mod.rs"]
pub(in crate::commands::core::web::implementation) mod native_environment;
#[path = "process_supervision/mod.rs"]
mod process_supervision;
pub(in crate::commands::core::web::implementation) use native_environment::*;
pub(super) use process_supervision::*;

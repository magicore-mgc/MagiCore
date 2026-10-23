//! Web build engine module exports.
//! Export module engine build Web.

#[path = "entry_point/mod.rs"]
mod entry_point;
#[path = "framework_script/mod.rs"]
mod framework_script;
#[cfg(feature = "web")]
#[path = "web_engine.rs"]
mod implementation;
#[path = "native_engine/mod.rs"]
mod native_engine;
#[path = "node_command/mod.rs"]
mod node_command;
#[path = "runtime_detection/mod.rs"]
mod runtime_detection;
#[path = "rust_build/mod.rs"]
mod rust_build;
#[path = "target/mod.rs"]
mod target;
#[path = "tool_execution/mod.rs"]
mod tool_execution;

pub(crate) use entry_point::find_entry_point;
pub(crate) use framework_script::run_framework_build_if_supported;
#[cfg(test)]
pub(crate) use framework_script::{
    map_framework_build_chain, map_framework_build_script, reject_external_package_manager_script,
};
#[cfg(feature = "web")]
pub(crate) use implementation::build_web;
pub(crate) use native_engine::build_native_engine;
pub(crate) use native_engine::find_native_engine_crate;
pub(crate) use node_command::{node_bin_args, prepend_path};
pub(crate) use runtime_detection::{detect_app_runtime, detect_lib_runtime};
pub(crate) use rust_build::{build_rust_with_env, build_rust_with_managed_source};
pub(crate) use target::{WebBuildTarget, resolve_web_build_target};
pub(crate) use tool_execution::{run_allowlisted_tool, run_allowlisted_tool_with_env};

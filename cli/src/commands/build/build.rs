//! Build command implementation wiring and compatibility exports.
use anyhow::{Result, bail};
use colored::Colorize;
use mgc_ui::info;
use std::path::{Path, PathBuf};

use crate::context::ProjectContext;

#[cfg(feature = "ai")]
#[path = "build/ai/mod.rs"]
mod ai;
#[cfg(feature = "app")]
#[path = "build/app/mod.rs"]
mod app;
#[cfg(feature = "clo")]
#[path = "build/cloud/mod.rs"]
mod cloud;
#[path = "build/dispatch/mod.rs"]
mod dispatch;
#[cfg(feature = "game")]
#[path = "build/game/mod.rs"]
mod game;
#[cfg(feature = "hardware")]
#[path = "build/hardware/mod.rs"]
mod hardware;
#[cfg(feature = "iot")]
#[path = "build/iot/mod.rs"]
mod iot;
#[cfg(feature = "lib")]
#[path = "build/lib/mod.rs"]
mod lib;
#[path = "build/shared/mod.rs"]
mod shared;
#[cfg(feature = "web")]
#[path = "build/web_go_backend/mod.rs"]
mod web_go_backend;

#[path = "web_engine/mod.rs"]
mod web_engine;

pub use dispatch::run;
pub(crate) use shared::{python_cmd, tool_unavailable};

#[cfg(feature = "ai")]
use ai::build_ai;
#[cfg(feature = "app")]
use app::build_app;
#[cfg(feature = "clo")]
use cloud::build_cloud;
#[cfg(all(test, feature = "clo"))]
pub(in crate::commands::build::implementation) use cloud::{
    cloud_cdk_synth_env, cloud_local_executable_exists, pulumi_preview_args,
};
#[cfg(feature = "game")]
use game::build_game;
#[cfg(feature = "hardware")]
use hardware::build_hardware;
#[cfg(feature = "iot")]
use iot::build_iot;
#[cfg(feature = "lib")]
use lib::build_lib;
use shared::{
    ensure_native_build_mode, find_root, go_offline_env, project_manifest_present,
    should_use_legacy_rust_build,
};
use web_engine::{
    build_rust_with_env, build_rust_with_managed_source, build_web, detect_app_runtime,
    detect_lib_runtime, prepend_path, run_allowlisted_tool, run_allowlisted_tool_with_env,
};
#[cfg(feature = "web")]
use web_go_backend::{GO_BACKEND_LANGUAGE, build_web_go_backend};

#[cfg(all(test, feature = "app"))]
pub(crate) use app::resolve_app_language;
#[cfg(all(test, feature = "app"))]
pub(crate) use app::{build_multi_app, finish_multi_build, resolve_multi_platforms};
#[cfg(test)]
pub(crate) use web_engine::*;
#[cfg(all(test, feature = "web"))]
use web_go_backend::ensure_go_build_output_dir;

#[cfg(test)]
#[path = "../../test/build_test.rs"]
mod tests;

pub(crate) use web_engine::node_bin_args;

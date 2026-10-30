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

#[path = "post_build/mod.rs"]
pub(crate) mod post_build;
#[path = "web_engine/mod.rs"]
mod web_engine;
pub(crate) use web_engine::EnginePreference;

pub use dispatch::run;

/// Build a Lib project through the same path `mgc build --core lib` uses.
///
/// `mgc dev --core lib` is a build-watch loop, so its rebuild must be the
/// product's build — not a second implementation that could drift from the one
/// a release uses. This wrapper exists only to widen visibility to the dev
/// owner; every check (freshness, managed source, post-build) stays inside
/// `build_lib`.
/// (`mgc dev --core lib` là build-watch nên rebuild của nó phải chính là build
/// của sản phẩm — không phải bản thứ hai có thể lệch. Wrapper này chỉ nới
/// visibility cho owner dev; mọi kiểm tra vẫn nằm trong `build_lib`.)
#[cfg(feature = "lib")]
pub(in crate::commands) async fn build_lib_for_dev(root: &std::path::Path) -> anyhow::Result<()> {
    build_lib(root, false).await
}
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
#[cfg(feature = "web")]
use web_engine::build_web;
use web_engine::{
    build_rust_with_env, build_rust_with_managed_source, detect_app_runtime, detect_lib_runtime,
    prepend_path, run_allowlisted_tool, run_allowlisted_tool_with_env,
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

/// Repair legacy Cargo registry sources before any managed Rust build.
/// Vá source registry Cargo legacy trước mọi build Rust managed.
pub(in crate::commands) fn repair_managed_sources(source_root: &Path) -> Result<usize> {
    web_engine::repair_managed_sources(source_root)
}

/// Build Rust from the MagiCore-managed source, repairing legacy stores first.
///
/// Every core that builds against the managed `registry/src` goes through this
/// one entry so the readiness-marker repair cannot be wired into only one of
/// them (`lib`, `iot`, `web` dev share the same store and the same failure).
/// Build Rust từ source do MagiCore quản lý, vá store legacy trước. Mọi core
/// build trên `registry/src` managed đều đi qua đúng một entry này để bước vá
/// marker không thể chỉ được nối vào một core (`lib`, `iot`, dev `web` dùng
/// chung store và cùng lỗi).
pub(in crate::commands) fn finish_managed_rust_build(
    root: &Path,
    optimizer_envs: std::collections::HashMap<String, String>,
    source_root: &Path,
) -> Result<()> {
    repair_managed_sources(source_root)?;
    build_rust_with_managed_source(root, optimizer_envs, source_root)
}

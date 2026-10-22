//! Stable runtime-detection API facade for optimizer and command callers.
//!
//! Implementations are kept under responsibility/core owners; this module only wires and re-exports them.

mod ai;
mod app;
mod cloud;
mod common;
mod lib;
mod package_manager;
mod python_manifest;
mod runtime_detect;
mod types;
mod web;

pub(crate) use python_manifest::python_project_declares_package;
pub use runtime_detect::{detect_optimizer_runtimes, detect_runtimes};
pub use types::{DetectedRuntime, PackageManager};

pub(super) use ai::detect_ai_runtime;
pub(super) use app::detect_app_runtime;
pub(super) use cloud::detect_cloud_node_runtime;
pub(super) use common::project_text;
pub(super) use lib::detect_lib_runtime;
pub(super) use package_manager::detect_package_manager;
pub(super) use web::detect_web_runtime;

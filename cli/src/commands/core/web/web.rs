use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use mgc_types::adapter::PackageAdapter;
use serde_json::{Map, Value};

use super::scaffold_flags::ScaffoldFlags;
use crate::commands::core::shared;
use mgc_types::Ecosystem;
use mgc_ui::info;

#[path = "versions/mod.rs"]
mod versions;
pub use versions::error as version_errors;
#[path = "workspace_layout/mod.rs"]
mod workspace_layout;
use workspace_layout::{
    WebProjectMode, detect_dev_target, detect_project_mode, install_targets, workspace_backend_dir,
    workspace_frontend_dir,
};
#[path = "dev/mod.rs"]
mod dev;
pub use dev::dev_at_root;
use dev::*;
#[path = "package_lifecycle/mod.rs"]
mod package_lifecycle;
pub use package_lifecycle::{add, install, list, remove, update};
#[path = "profiling/mod.rs"]
mod profiling;
use profiling::*;
#[path = "project_context/mod.rs"]
mod project_context;
use project_context::*;
#[path = "create/mod.rs"]
mod create;
use create::*;
#[path = "feature_selection/mod.rs"]
mod feature_selection;
use feature_selection::*;
#[path = "framework_request/mod.rs"]
mod framework_request;
use framework_request::*;
#[path = "framework_catalog/mod.rs"]
mod framework_catalog;
use framework_catalog::*;
#[path = "feature_catalog/mod.rs"]
mod feature_catalog;
use feature_catalog::*;
#[path = "manifest_seed/mod.rs"]
mod manifest_seed;
#[cfg(test)]
use manifest_seed::{ensure_primary_package, resolve_primary_version};

pub use create::run_create_with_options;
pub(crate) use framework_request::{FrameworkRequest, parse_framework_request};
pub(crate) use manifest_seed::enrich_web_project_manifest;
pub use project_context::check_dev_freshness;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

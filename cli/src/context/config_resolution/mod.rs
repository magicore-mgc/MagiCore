//! Resolve project configuration through focused identity owners.
//! Phân giải cấu hình project qua các owner identity chuyên trách.

mod config_resolution;
mod configured_project;
mod core_identity;
mod detected_project;
mod error;
mod unclaimed_project;

pub(super) use config_resolution::resolve_config;
use core_identity::{ensure_override_matches_identity, project_name};
use error::CoreOverrideConflictError;

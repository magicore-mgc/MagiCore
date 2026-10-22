//! AI framework detection module wiring.
//! Wiring module nhận diện framework AI.

mod detection;
mod framework_name;
mod project_config;
mod pyproject;

use super::types::AiFramework;

pub(super) use detection::detect_framework;
pub(super) use framework_name::framework_from_str;
pub(super) use project_config::framework_from_project_config;
pub(super) use pyproject::framework_from_pyproject;

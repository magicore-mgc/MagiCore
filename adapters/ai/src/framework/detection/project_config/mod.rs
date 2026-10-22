//! Project-config AI framework reader wiring.
//! Wiring module reader framework từ project config.

mod project_config;

pub(in crate::framework) use project_config::framework_from_project_config;

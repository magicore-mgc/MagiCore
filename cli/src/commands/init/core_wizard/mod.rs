//! Core wizard owner exports.
//! Export owner định tuyến wizard core.

mod core_wizard;

pub(super) use super::project_name::ask_project_name;
pub(super) use super::web_features::ask_web_features;
pub(in crate::commands::init) use core_wizard::{iot_wizard_config, pick_core, run_core_wizard};

//! Wires Web configuration, framework-selection and flag-policy owners.
//! Nối các owner cấu hình Web, chọn framework và policy flag.

use super::*;

#[path = "flag_policy/mod.rs"]
mod flag_policy;
#[path = "framework_selection/mod.rs"]
mod framework_selection;
#[path = "configuration.rs"]
mod implementation;
#[path = "offline_combo/mod.rs"]
mod offline_combo;

pub(in crate::commands::core::web::implementation) use self::flag_policy::{
    apply_preset, validate_flags,
};
pub(in crate::commands::core::web::implementation) use self::framework_selection::{
    detect_backend_framework, enforce_framework_language_defaults, resolve_framework,
};
pub(in crate::commands::core::web::implementation) use self::implementation::{
    build_web_config, required_web_layers_for_config,
};
pub(in crate::commands::core::web::implementation) use self::offline_combo::{
    compose_offline, is_combo,
};

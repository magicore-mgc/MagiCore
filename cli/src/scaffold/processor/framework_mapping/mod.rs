// framework_mapping owner wiring — nối owner framework_mapping.
// Keep the implementation and re-export boundary small.
use super::*;
#[path = "framework_mapping.rs"]
mod implementation;
pub(in crate::scaffold) use implementation::fullstack_frontend_framework;
pub(in crate::scaffold::processor) use implementation::{
    effective_web_mode, framework_uses_react_shell, fullstack_backend_framework,
    fullstack_uses_frontend_foundation, fullstack_uses_react_shell, legacy_template_target,
    normalize_web_framework,
};
pub use implementation::{infer_backend_language, is_all_in_one_fullstack};

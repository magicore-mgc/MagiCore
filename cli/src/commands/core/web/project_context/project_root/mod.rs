// project_root owner wiring — nối owner project_root.
// Re-export only to the intended parent boundary.
use super::*;
#[path = "project_root.rs"]
mod implementation;
pub(in crate::commands::core::web::implementation) use implementation::*;

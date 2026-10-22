// adapter_selection owner wiring — nối owner adapter_selection.
// Re-export only to the intended parent boundary.
use super::*;
#[path = "adapter_selection.rs"]
mod implementation;
pub(in crate::commands::core::web::implementation) use implementation::*;

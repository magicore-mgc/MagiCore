// initializer owner wiring — nối owner initializer.
// Re-export only to the intended parent boundary.
use super::*;
#[path = "initializer.rs"]
mod implementation;
pub(in crate::commands::core::web::implementation) use implementation::run_create_initializer;

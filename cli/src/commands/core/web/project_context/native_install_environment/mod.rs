// native_install_environment owner wiring — nối owner native_install_environment.
// Re-export only to the intended parent boundary.
use super::*;
#[path = "native_install_environment.rs"]
mod implementation;
pub(in crate::commands::core::web::implementation) use implementation::*;

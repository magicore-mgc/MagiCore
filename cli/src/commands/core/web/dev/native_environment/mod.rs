// native_environment owner wiring — nối owner native_environment.
// native_environment module export stays focused at this boundary.
use super::*;
#[path = "native_environment.rs"]
mod implementation;
pub(in crate::commands::core::web::implementation) use implementation::*;

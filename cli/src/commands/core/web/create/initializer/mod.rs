// initializer owner wiring — nối owner initializer.
// Re-export only to the intended parent boundary.
use super::*;
#[path = "host_tool/mod.rs"]
mod host_tool;
#[path = "http_archive/mod.rs"]
mod http_archive;
#[path = "initializer.rs"]
mod implementation;
pub(in crate::commands::core::web::implementation) use host_tool::run_host_tool;
pub(in crate::commands::core::web::implementation) use http_archive::run_http_archive;
pub(in crate::commands::core::web::implementation) use implementation::run_create_initializer;

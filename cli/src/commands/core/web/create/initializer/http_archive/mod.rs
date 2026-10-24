//! HTTP archive initializer owner.
//! Owner initializer archive HTTP.

#[path = "http_archive.rs"]
mod implementation;

pub(in crate::commands::core::web::implementation) use implementation::run_http_archive;

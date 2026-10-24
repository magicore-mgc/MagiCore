//! Host-tool initializer owner.
//! Owner initializer chương trình có sẵn trên máy.

#[path = "host_tool.rs"]
mod implementation;

pub(in crate::commands::core::web::implementation) use implementation::run_host_tool;

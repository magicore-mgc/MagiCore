//! Command-line initializers for cores without a registry layer.
//! Initializer CLI cho các core không có registry layer.

#[path = "catalog/mod.rs"]
mod catalog;
#[path = "host_tool/mod.rs"]
mod host_tool;
#[path = "types/mod.rs"]
mod types;

pub(crate) use catalog::RECORDS;
pub(crate) use host_tool::run as run_host_tool;
#[cfg(test)]
pub(crate) use types::CoreInitializer;

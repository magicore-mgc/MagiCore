//! Wires language-specific native dev launch owners and shared input.
//! Nối owner native dev theo ngôn ngữ và input dùng chung.

mod environment;
mod go;
mod java;
mod native_launch;
mod php;
mod python;
mod rust;
mod types;

pub(in crate::commands::core::web::implementation) use native_launch::infer_native_dev_launch;

#[cfg(test)]
pub(in crate::commands::core::web::implementation) use rust::cargo_run_args_from_managed_source;

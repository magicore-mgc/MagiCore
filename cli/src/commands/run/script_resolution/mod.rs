//! Script source-resolution owner wiring.
//! Wiring owner phân giải nguồn script.

mod script_resolution;
pub(crate) use script_resolution::resolve_task_script;
#[cfg(test)]
pub(crate) use script_resolution::{resolve_mgc_toml_script, resolve_package_json_script};

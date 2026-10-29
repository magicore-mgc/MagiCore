//! Script execution-policy owner wiring.
//! Wiring owner policy thực thi script.

mod script_policy;
#[cfg(test)]
pub(in crate::commands::run) use script_policy::validate_runtime_flags;
pub(super) use script_policy::{gate_script_program, reject_external_package_manager_script};

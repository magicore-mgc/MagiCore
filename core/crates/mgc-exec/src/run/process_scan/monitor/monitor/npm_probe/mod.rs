//! npm probe wiring.
//! Wiring npm probe.

mod command_line;
mod npm_probe;
mod verified_shim;
pub(super) use command_line::{
    generated_npm_probe_shim_from_command_line, npm_probe_command_line_matches_shim_path,
};
pub(super) use npm_probe::is_allowed_shadow_npm_version_probe;
#[cfg(test)]
pub(super) use npm_probe::{is_generated_shadow_npm_version_probe, is_shadow_npm_version_probe};
pub(super) use verified_shim::is_verified_shadow_npm_probe_shim;

//! Wires the private package_resolved_pins owner modules.
//! Nối các module private thuộc owner package_resolved_pins.

mod git_pin;
mod package_resolved_pins;
mod registry_pin;

pub(super) use git_pin::build_git_pin;
pub(in crate::install) use package_resolved_pins::build_swift_pins;
pub(super) use registry_pin::build_registry_pin;

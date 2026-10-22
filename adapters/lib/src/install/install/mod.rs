//! Lib install coordinator owner wiring.
//! Wiring owner điều phối install Lib.

mod empty_graph;
mod install;
mod offline_policy;
mod preflight;

pub(super) use empty_graph::install_empty_resolved_graph;
pub(crate) use install::run_install;
pub(super) use offline_policy::validate_offline_install;
pub(super) use preflight::validate_native_install_inputs;

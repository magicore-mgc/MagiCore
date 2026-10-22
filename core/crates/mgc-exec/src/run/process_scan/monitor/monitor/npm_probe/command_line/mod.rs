//! npm probe command-line wiring.
//! Wiring command line của npm probe.

mod command_line;
pub(in crate::run::implementation::process_scan::monitor::implementation) use command_line::{
    generated_npm_probe_shim_from_command_line, npm_probe_command_line_matches_shim_path,
};

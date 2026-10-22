//! Monitored child command-line validation exports.
//! Export validation command line của process con được monitor.

#[path = "child_command_line.rs"]
mod implementation;
pub(in crate::run::implementation) use implementation::validate_monitored_child_command_line;

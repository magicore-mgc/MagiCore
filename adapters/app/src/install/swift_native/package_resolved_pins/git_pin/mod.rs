//! Wires the private git_pin owner modules.
//! Nối các module private thuộc owner git_pin.

mod git_pin;
pub(in crate::install::swift_native) use git_pin::build_git_pin;

//! Wires the nuget native install owner.
//! Wiring owner nuget native install.

pub(super) mod error;
mod nuget_native_install;

pub(super) use nuget_native_install::install_nuget_native;

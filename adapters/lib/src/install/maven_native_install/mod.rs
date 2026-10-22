//! Wires the maven native install owner.
//! Wiring owner maven native install.

pub(super) mod error;
mod maven_native_install;

pub(super) use maven_native_install::install_maven_native;

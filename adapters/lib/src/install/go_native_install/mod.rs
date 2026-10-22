//! Wires the go native install owner.
//! Wiring owner go native install.

pub(super) mod error;
mod go_native_install;

pub(super) use go_native_install::install_go_native;

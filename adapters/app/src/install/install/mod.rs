//! Install coordinator owner module.
//! Module chủ quản điều phối install.

mod flutter;
mod install;
mod language_gate;
mod swift;

pub use install::run_install;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(crate) use install::run_install_with_swift_protocol;

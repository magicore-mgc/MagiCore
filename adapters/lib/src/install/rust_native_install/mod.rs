//! Wires the rust native install owner.
//! Wiring owner rust native install.

mod artifact_cache;
pub(crate) mod error;
mod offline_install;
mod online_install;
mod rust_native_install;
mod source_readiness;

pub(crate) use self::artifact_cache::read_cached_rust_artifact;
#[cfg(test)]
pub(crate) use self::offline_install::{install_rust_cached, preflight_rust_artifact_cache};
pub(super) use self::rust_native_install::install_rust_native;
pub(in crate::install) use self::source_readiness::require_cargo_source_marker;

#[cfg(test)]
#[path = "test/mod.rs"]
pub(super) mod test;

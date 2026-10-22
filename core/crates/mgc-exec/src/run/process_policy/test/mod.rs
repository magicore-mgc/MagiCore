//! Matcher test wiring.
//! Wiring test matcher.

use super::*;
#[cfg(test)]
use crate::run::implementation::{
    is_go_toolchain_version_probe, reject_external_dependency_resolution,
};

#[path = "go_environment.rs"]
mod go_environment;
#[path = "go_toolchain_probe.rs"]
mod go_toolchain_probe;
#[path = "metadata.rs"]
mod metadata;
#[path = "runtime_entrypoints.rs"]
mod runtime_entrypoints;

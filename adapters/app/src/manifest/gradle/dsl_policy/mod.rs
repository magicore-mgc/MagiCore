//! Gradle DSL policy module wiring.
//! Wiring module policy DSL Gradle.

mod dsl_policy;

pub(super) use dsl_policy::{parse_error, write_error};

//! TOML manifest parser wiring — nối parser Cargo và Python.
//! Wiring parser manifest TOML — nối Cargo và Python.

#[path = "toml_manifest.rs"]
mod toml_manifest;

pub(in crate::dispatch) use toml_manifest::{cargo_core, python_core};

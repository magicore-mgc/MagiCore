//! Lib manifest classification module wiring.
//! Wiring module phân loại manifest Lib.

mod manifest_classification;
mod probe_cargo_toml;
mod probe_package_json;
mod probe_pyproject;

pub(crate) use manifest_classification::manifest_is_lib;

//! Node manifest parser wiring — nối parser package.json.
//! Wiring parser Node — nối parser package.json.

#[path = "package_json.rs"]
mod package_json;

pub(in crate::dispatch) use package_json::declared_core;

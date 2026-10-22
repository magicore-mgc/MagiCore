//! package.json declaration owner wiring.
//! Wiring owner khai báo package.json.
mod package_json;
pub(super) use package_json::explicit_core;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

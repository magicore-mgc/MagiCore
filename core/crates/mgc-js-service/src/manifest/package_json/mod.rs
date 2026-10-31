//! PackageJson schema and IO exports.
//! Export schema và IO PackageJson.

mod package_json;

#[cfg(test)]
pub(super) use package_json::MAX_MANIFEST_SIZE;
pub use package_json::PackageJson;
pub(super) use package_json::load_checked;
#[cfg(test)]
pub(super) use package_json::serialize_package_json;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

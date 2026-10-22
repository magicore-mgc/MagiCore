//! Wire selected SDK package manifest and dependency projection.
//! Nối manifest package SDK được chọn và projection dependency.

mod sdk_dependencies;
mod sdk_manifest;
mod sdk_packages;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use self::sdk_packages::read_sdk_package_entries;

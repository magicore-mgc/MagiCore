//! Flutter SDK manifest-closure owner wiring.
//! Wiring owner closure manifest Flutter SDK.

mod declared_dependencies;
mod dependency_closure;
mod package_names;
mod project_dependencies;
mod registry_dependency;
mod sdk_package_manifest;

pub(super) use declared_dependencies::declared_flutter_sdk_dependencies;
pub(super) use dependency_closure::read_flutter_sdk_dependency_closure;
pub(crate) use package_names::flutter_sdk_package_names;

#[cfg(test)]
mod test;

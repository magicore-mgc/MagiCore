//! Flutter SDK integration wiring.
//! Wiring tích hợp Flutter SDK.

mod discovery;
mod expand;
mod manifest_closure;
mod package_root;
mod path_lookup;

#[cfg(test)]
pub(crate) use expand::expand_flutter_sdk_dependencies_with_root;
pub(crate) use expand::{
    expand_flutter_sdk_dependencies, flutter_sdk_packages, has_flutter_sdk_dependencies,
};
pub(crate) use manifest_closure::flutter_sdk_package_names;
pub(crate) use package_root::flutter_sdk_package_root;
#[cfg(test)]
pub(crate) use path_lookup::find_flutter_executable_in_path;

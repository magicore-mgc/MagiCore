//! Swift-native install owner.
//! Module chủ quản cài Swift native.
mod git_checkout;
mod git_provenance;
mod package_resolved_pins;
mod package_source;
mod registry_archive;
mod store_root;
mod swift_native;
pub(super) use git_checkout::materialize_git_checkout;
pub(super) use git_provenance::git_commit_for_package;
pub(super) use package_resolved_pins::build_swift_pins;
pub(super) use package_source::{SwiftPackageSource, swift_package_source};
pub(super) use registry_archive::install_registry_archive;
pub(super) use store_root::swift_store_root;
pub(crate) use swift_native::install_swift_native;

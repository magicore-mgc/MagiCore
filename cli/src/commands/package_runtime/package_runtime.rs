//! Compatibility facade for shared package-runtime callers.
//! Facade tương thích cho các caller runtime package dùng chung.

pub(crate) use super::bin_selection::{select_bin_name, valid_bin_name};
pub(crate) use super::install_policy::{registry_install_args, registry_project_config};
pub(crate) use super::installed_metadata::requested_package_bins;
pub(crate) use super::request_validation::parse_request;
pub(crate) use super::run_registry_package::run_registry_package_in;
pub(crate) use super::work_root::PackageRunPurpose;

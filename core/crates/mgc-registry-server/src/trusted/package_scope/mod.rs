mod package_scope;
pub use package_scope::{canonical_pypi_name, core_scoped_oci_repository, scoped_package};

pub(in crate::trusted::trusted) use package_scope::validate_repository;

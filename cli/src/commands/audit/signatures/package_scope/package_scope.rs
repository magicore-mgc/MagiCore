//! Map only OCI packages to the canonical core namespace.
//! Chỉ ánh xạ package OCI sang namespace core chuẩn.

use super::error::PackageScopeError;

/// Add a canonical core namespace when auditing a trusted OCI artifact repository.
/// Thêm namespace core chuẩn khi audit repository artifact OCI trusted.
pub(in crate::commands::audit::signatures) fn effective_signature_package(
    core: Option<&str>,
    protocol: &str,
    package: &str,
) -> std::result::Result<String, PackageScopeError> {
    match (protocol, core) {
        ("oci", Some(core)) => {
            mgc_registry_server::trusted::core_scoped_oci_repository(core, package)
                .map_err(|_| PackageScopeError)
        }
        _ => Ok(package.to_owned()),
    }
}

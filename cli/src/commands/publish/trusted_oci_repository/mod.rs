mod trusted_oci_repository;
pub(in crate::commands) use trusted_oci_repository::{
    effective_trusted_oci_repository, oci_artifact_filename, trusted_oci_registry_root,
};

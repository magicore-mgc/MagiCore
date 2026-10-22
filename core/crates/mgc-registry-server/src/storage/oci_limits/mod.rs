mod oci_limits;
pub(crate) use oci_limits::max_oci_blob_bytes;
pub(crate) use oci_limits::max_oci_manifest_verify_bytes;

pub(in crate::storage) use oci_limits::validate_oci_blob_size;

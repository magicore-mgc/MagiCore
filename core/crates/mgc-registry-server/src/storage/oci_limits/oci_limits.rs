//! Centralizes OCI blob and manifest verification byte limits and boundary validation.
//! Tập trung giới hạn byte OCI blob/manifest và kiểm tra ranh giới.

use super::super::prelude::*;
use super::super::*;

const DEFAULT_MAX_OCI_BLOB_BYTES: u64 = 20 * 1024 * 1024 * 1024;

const DEFAULT_MAX_OCI_MANIFEST_VERIFY_BYTES: u64 = 40 * 1024 * 1024 * 1024;

pub(crate) fn max_oci_blob_bytes() -> u64 {
    std::env::var("MGC_REGISTRY_MAX_OCI_BLOB_BYTES")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|limit| *limit > 0)
        .unwrap_or(DEFAULT_MAX_OCI_BLOB_BYTES)
}

pub(crate) fn max_oci_manifest_verify_bytes() -> u64 {
    std::env::var("MGC_REGISTRY_MAX_OCI_MANIFEST_VERIFY_BYTES")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|limit| *limit > 0)
        .unwrap_or(DEFAULT_MAX_OCI_MANIFEST_VERIFY_BYTES)
}

pub(in crate::storage) fn validate_oci_blob_size(
    size: u64,
    limit: u64,
) -> Result<(), OciUploadAppendError> {
    if size > limit {
        Err(OciUploadAppendError::TooLarge)
    } else {
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/oci_limits.rs"]
mod tests;

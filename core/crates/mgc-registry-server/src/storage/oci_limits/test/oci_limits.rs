//! OCI upload limit boundary regression.
//! Kiểm thử ranh giới giới hạn OCI upload.

use super::*;

#[test]
fn oci_upload_limit_accepts_boundary_and_rejects_next_byte() {
    assert!(validate_oci_blob_size(8, 8).is_ok());
    assert!(matches!(
        validate_oci_blob_size(9, 8),
        Err(OciUploadAppendError::TooLarge)
    ));
}

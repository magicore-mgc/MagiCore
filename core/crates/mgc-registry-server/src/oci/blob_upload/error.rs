use super::super::prelude::StatusCode;

/// Map storage append failures to their OCI HTTP status.
/// Chuyển lỗi append storage sang status HTTP OCI tương ứng.
pub(super) fn oci_append_error_status(error: crate::storage::OciUploadAppendError) -> StatusCode {
    match error {
        crate::storage::OciUploadAppendError::SessionNotFound => StatusCode::NOT_FOUND,
        crate::storage::OciUploadAppendError::OffsetMismatch => StatusCode::RANGE_NOT_SATISFIABLE,
        crate::storage::OciUploadAppendError::TooLarge => StatusCode::PAYLOAD_TOO_LARGE,
        crate::storage::OciUploadAppendError::Storage(error) => {
            tracing::error!("OCI upload chunk storage failed: {error:#}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

//! Preserves the JSON serializer error as a typed doctor-local source.
//! Giữ lỗi serializer JSON dưới dạng source có kiểu riêng cho doctor.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::doctor) enum DoctorError {
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

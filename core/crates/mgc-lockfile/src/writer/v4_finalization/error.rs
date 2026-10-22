//! Classifies canonical v4 serializer drift.
//! Phân loại lỗi serializer v4 canonical bị lệch.

#[derive(Debug, thiserror::Error)]
pub(super) enum V4FinalizationError {
    #[error("v4 serializer drift: emitted document does not round-trip")]
    SerializerDrift,
}

impl From<V4FinalizationError> for crate::LockfileError {
    fn from(error: V4FinalizationError) -> Self {
        Self::WriteFailed(error.to_string())
    }
}

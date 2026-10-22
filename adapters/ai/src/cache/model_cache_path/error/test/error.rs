//! Model path error case — xác nhận lỗi validation được chuyển qua MgError.

#![cfg(test)]

use super::error::ModelCachePathError;
use mgc_types::MgError;

#[test]
fn invalid_model_id_error_keeps_typed_diagnostic() {
    let error = MgError::from(ModelCachePathError::InvalidModelId);

    assert!(error.to_string().contains("unsafe path segment"));
    let MgError::Anyhow(cause) = error else {
        panic!("expected typed error boundary");
    };
    assert!(matches!(
        cause.downcast_ref::<ModelCachePathError>(),
        Some(ModelCachePathError::InvalidModelId)
    ));
}

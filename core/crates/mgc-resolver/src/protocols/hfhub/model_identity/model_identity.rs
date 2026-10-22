//! Hugging Face model identity validation.
//! Kiểm tra định danh model Hugging Face.

use super::error::ModelIdentityError;

/// Validate a repo id as exactly one org/name pair.
/// Kiểm tra repo id đúng một cặp org/name.
pub(in crate::protocols::hfhub) fn check_model_id(model: &str) -> Result<(), ModelIdentityError> {
    let valid = !model.is_empty()
        && model.matches('/').count() == 1
        && !model.contains("..")
        && !model.contains("://")
        && model
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '/'));
    if valid {
        Ok(())
    } else {
        Err(ModelIdentityError::InvalidModelId(model.to_string()))
    }
}

/// Require an exact immutable 40-hex commit revision.
/// Bắt buộc revision là commit bất biến 40-hex chính xác.
pub(in crate::protocols::hfhub) fn check_revision(
    revision: &str,
) -> Result<(), ModelIdentityError> {
    if revision.len() == 40 && revision.chars().all(|c| c.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err(ModelIdentityError::InvalidRevision(revision.to_string()))
    }
}

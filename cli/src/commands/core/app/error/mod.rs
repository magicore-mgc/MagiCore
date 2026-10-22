//! App-local typed-error owner exports.
//! Export owner lỗi có kiểu cục bộ của App.

mod error;

pub(super) use error::AppCoreError;
pub(crate) use error::{native_lifecycle_error, project_not_detected_error};

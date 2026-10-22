//! Path validation module exports.
//! Export module kiểm tra đường dẫn.

mod error;
mod path_validation;

pub(crate) use error::PathValidationError;
pub(super) use path_validation::{safe_vstore_target, validate_patch_relative_path};

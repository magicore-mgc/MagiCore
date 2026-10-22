//! Wires source-tree and symlink-target validation.
//! Nối owner xác thực source tree và symlink target.

mod error;
mod source_validation;

pub(super) use error::SourceValidationError;
pub(in crate::commands::store) use source_validation::{
    ValidatedSourceTree, validate_present_source_tree, validate_real_directory,
    validated_link_target,
};

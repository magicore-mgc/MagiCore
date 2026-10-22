//! Validation facade — export project argument validation.
//! Facade xác thực — export kiểm tra argument theo project.

#[path = "validation.rs"]
mod implementation;
pub(super) use implementation::validate_args_no_traversal;

//! Relative model artifact path validation.
//! Kiểm tra đường dẫn tương đối của artifact model.

mod model_path;
pub(in crate::install::download) use model_path::validate_model_relative_path;

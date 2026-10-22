//! Shared JSONC normalization owners.
//! Các owner chuẩn hóa JSONC dùng chung.

mod jsonc;
mod line_comment;
mod trailing_comma;

pub use self::jsonc::strip_jsonc;

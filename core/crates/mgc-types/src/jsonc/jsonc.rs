//! Coordinate the shared JSONC cleanup steps.
//! Điều phối các bước dọn JSONC dùng chung.

use super::{line_comment::strip_line_comments, trailing_comma::strip_trailing_commas};

/// Remove JSONC line comments and trailing commas before strict JSON parsing.
/// Bỏ comment dòng và dấu phẩy cuối trước khi consumer parse JSON strict.
pub fn strip_jsonc(raw: &str) -> String {
    let comments_removed = strip_line_comments(raw);
    strip_trailing_commas(&comments_removed)
}

#[cfg(test)]
#[path = "test/jsonc.rs"]
mod tests;

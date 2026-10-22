//! Hidden password interaction — tương tác nhập password ẩn.
//! Dialoguer errors continue to propagate through anyhow::Result.
//! Lỗi dialoguer tiếp tục được propagate qua anyhow::Result.

use crate::prompt::style_prompt;
use anyhow::Result;
use dialoguer::Password;

/// Read a password without echo — nhập password mà không echo.
/// The dialoguer password interaction remains unchanged — tương tác password dialoguer giữ nguyên.
pub fn password(prompt: &str) -> Result<String> {
    Ok(Password::new()
        .with_prompt(style_prompt(prompt))
        .interact()?)
}

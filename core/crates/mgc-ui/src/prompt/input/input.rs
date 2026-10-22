//! Text-input interaction — tương tác nhập văn bản.
//! Terminal input errors keep their existing anyhow propagation.
//! Lỗi nhập terminal giữ nguyên propagation anyhow.

use crate::prompt::style_prompt;
use anyhow::Result;
use dialoguer::Input;

/// Ask for text input — hỏi dữ liệu dạng text.
/// The dialoguer text interaction remains unchanged — tương tác text dialoguer giữ nguyên.
pub fn input(prompt: &str) -> Result<String> {
    Ok(Input::new()
        .with_prompt(style_prompt(prompt))
        .interact_text()?)
}

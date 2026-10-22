//! Yes/no prompt — prompt yes/no.
//! Dialoguer errors continue to propagate through anyhow::Result.
//! Lỗi dialoguer tiếp tục được propagate qua anyhow::Result.

use crate::prompt::style_prompt;
use anyhow::Result;
use dialoguer::Confirm;

/// Ask a yes/no question — hỏi câu yes/no.
/// The existing default answer remains yes and is shown — giữ mặc định yes và hiển thị mặc định.
pub fn confirm(prompt: &str) -> Result<bool> {
    Ok(Confirm::new()
        .with_prompt(style_prompt(prompt))
        .default(true)
        .show_default(true)
        .interact()?)
}

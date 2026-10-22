//! Shared prompt-style wiring — wiring style prompt dùng chung.
//! Keeps the shared helper private to this crate's prompt API.
//! Giữ helper style dùng chung ở phạm vi nội bộ crate prompt.

mod prompt_style;
#[cfg(test)]
#[path = "test/prompt_style.rs"]
mod tests;

pub(super) use prompt_style::style_prompt;

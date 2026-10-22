//! Multi-item selection — chọn nhiều item.
//! The original checkbox and white-text styling are preserved.
//! Giữ nguyên checkbox và style chữ trắng.

use crate::prompt::style_prompt;
use anyhow::Result;
use console::style;
use dialoguer::MultiSelect;

pub(super) fn styled_options(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .map(|item| format!("{} {}", style("☐").dim(), style(item).white()))
        .collect()
}

/// Select multiple items — chọn nhiều item.
/// The dialoguer result and item order remain unchanged — giữ result và thứ tự item dialoguer.
pub fn multi_select(prompt: &str, items: &[&str]) -> Result<Vec<usize>> {
    let selected = styled_options(items);
    let display: Vec<&str> = selected.iter().map(|item| item.as_str()).collect();
    Ok(MultiSelect::new()
        .with_prompt(style_prompt(prompt))
        .items(&display)
        .interact()?)
}

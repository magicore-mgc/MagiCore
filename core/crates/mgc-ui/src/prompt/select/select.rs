//! Single-item selection — chọn một item.
//! The visible option builder is shared by the production interaction and tests.
//! Builder option hiển thị được dùng chung bởi tương tác production và test.

use crate::prompt::style_prompt;
use anyhow::Result;
use console::style;
use dialoguer::Select;

pub(super) fn styled_options(items: &[&str]) -> Vec<String> {
    items.iter().map(|i| style(i).white().to_string()).collect()
}

/// Select one item — chọn một item.
/// The original zero-based default remains selected — giữ lựa chọn mặc định index 0.
pub fn select(prompt: &str, items: &[&str]) -> Result<usize> {
    let styled = styled_options(items);
    let refs: Vec<&str> = styled.iter().map(|item| item.as_str()).collect();
    Ok(Select::new()
        .with_prompt(style_prompt(prompt))
        .items(&refs)
        .default(0)
        .interact()?)
}

//! Shared prompt-label styling — style label prompt dùng chung.
//! This pure helper is consumed by every interactive prompt owner.
//! Helper thuần này được mọi owner prompt tương tác dùng.

use console::style;

pub(in crate::prompt) fn style_prompt(text: &str) -> String {
    format!(
        "{} {}",
        style("◆").cyan().bold(),
        style(text).white().bold()
    )
}

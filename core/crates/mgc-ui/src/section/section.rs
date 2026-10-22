//! Step-section output — output section từng bước.
//! Line construction is separated from writing so its text contract is testable.
//! Tách dựng dòng khỏi thao tác in để kiểm thử được contract nội dung.

use crate::quiet::is_quiet;
use console::style;

pub(super) fn section_lines(title: &str, current: usize, total: usize) -> [String; 3] {
    [
        format!(
            "{} {}",
            style("◆").cyan().bold(),
            style(format!("Step {}/{}", current, total)).dim()
        ),
        format!("{} {}", style("┃").cyan(), style(title).bold().white()),
        format!("{} {}", style("┃").cyan(), style("─".repeat(40)).dim()),
    ]
}

/// Print a section header — in heading section.
/// Quiet mode suppresses the whole section — quiet sẽ ẩn toàn bộ section.
pub fn section(title: &str, current: usize, total: usize) {
    if is_quiet() {
        return;
    }
    println!();
    for line in section_lines(title, current, total) {
        println!("  {}", line);
    }
}

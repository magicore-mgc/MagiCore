//! Install summary formatting — định dạng summary cài đặt.
//! Rows are built once and then printed in the same original order.
//! Dòng được dựng một lần rồi in theo đúng thứ tự ban đầu.

use crate::quiet::is_quiet;
use console::style;

pub(super) fn install_summary_lines(
    added: usize,
    cached: usize,
    duration_ms: u64,
    disk_saved: &str,
    cache_source: Option<&str>,
) -> Vec<String> {
    let mut lines = vec![
        format!(
            "{}",
            style("┌──────────────────────────────────────────────┐").cyan()
        ),
        format!(
            "{}",
            style("│           📦  Install Complete               │")
                .cyan()
                .bold()
        ),
        format!(
            "{}",
            style("├──────────────────────────────────────────────┤").cyan()
        ),
        format!("{} {:>3} packages installed", style("│").cyan(), added),
        format!("{} {:>3} from cache", style("│").cyan(), cached),
    ];
    if let Some(source) = cache_source {
        lines.push(format!(
            "{}   cache source: {}",
            style("│").cyan(),
            style(source).dim()
        ));
    }
    lines.push(format!("{} {:>5} ms total", style("│").cyan(), duration_ms));
    lines.push(format!(
        "{} {:>10} saved (CAS dedup)",
        style("│").cyan(),
        style(disk_saved).green().bold()
    ));
    lines.push(format!(
        "{}",
        style("└──────────────────────────────────────────────┘").cyan()
    ));
    lines
}

/// Print the install summary — in summary cài đặt.
/// The legacy helper omits the cache-source row — helper cũ bỏ dòng nguồn cache.
pub fn print_install_summary(added: usize, cached: usize, duration_ms: u64, disk_saved: &str) {
    print_install_summary_source(added, cached, duration_ms, disk_saved, None)
}

/// Print install summary with cache provenance — in summary kèm nguồn cache.
/// This label states where cache bytes came from — nhãn nêu byte cache đến từ đâu.
/// Zero cached bytes must not imply that cache is unused.
/// Zero byte không đồng nghĩa với việc cache không được dùng.
pub fn print_install_summary_source(
    added: usize,
    cached: usize,
    duration_ms: u64,
    disk_saved: &str,
    cache_source: Option<&str>,
) {
    if is_quiet() {
        return;
    }
    println!();
    for line in install_summary_lines(added, cached, duration_ms, disk_saved, cache_source) {
        println!("  {}", line);
    }
}

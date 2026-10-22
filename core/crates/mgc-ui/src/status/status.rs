//! Terminal status messages — thông báo trạng thái terminal.
//! Text builders keep output formatting testable while printers retain quiet rules.
//! Builder nội dung giúp kiểm thử format, printer giữ nguyên quy tắc quiet.

use crate::quiet::is_quiet;
use console::style;

pub(super) fn success_line(msg: &str) -> String {
    format!("  {} {}", style("✔").green().bold(), msg)
}

pub(super) fn error_line(msg: &str) -> String {
    format!("  {} {}", style("✘").red().bold(), msg)
}

pub(super) fn warning_line(msg: &str) -> String {
    format!("  {} {}", style("WARN:").yellow().bold(), msg)
}

pub(super) fn info_line(msg: &str) -> String {
    format!("  {} {}", style("ℹ").blue(), msg)
}

pub(super) fn hint_line(msg: &str) -> String {
    format!("  {}", style(msg).dim())
}

/// Print a success message — in thông báo thành công.
/// Quiet mode suppresses this message — quiet sẽ ẩn thông báo này.
pub fn success(msg: &str) {
    if is_quiet() {
        return;
    }
    println!("{}", success_line(msg));
}

/// Print an error message — in thông báo lỗi.
/// Errors remain visible in quiet mode — lỗi vẫn hiển thị khi quiet.
pub fn error(msg: &str) {
    eprintln!("{}", error_line(msg));
}

/// Print a warning message — in cảnh báo.
/// Quiet mode suppresses this message — quiet sẽ ẩn cảnh báo.
pub fn warning(msg: &str) {
    if is_quiet() {
        return;
    }
    println!("{}", warning_line(msg));
}

/// Print a warning to stderr when stdout carries machine-readable output.
/// In cảnh báo ra stderr khi stdout đang mang dữ liệu máy đọc.
pub fn warning_stderr(msg: &str) {
    if is_quiet() {
        return;
    }
    eprintln!("{}", warning_line(msg));
}

/// Print an informational message — in thông tin.
/// Quiet mode suppresses this message — quiet sẽ ẩn thông tin.
pub fn info(msg: &str) {
    if is_quiet() {
        return;
    }
    println!("{}", info_line(msg));
}

/// Print a dimmed hint — in gợi ý mờ.
/// Quiet mode suppresses this message — quiet sẽ ẩn gợi ý.
pub fn hint(msg: &str) {
    if is_quiet() {
        return;
    }
    println!("{}", hint_line(msg));
}

/// Print a blank line — in dòng trống.
/// Quiet mode suppresses this output — quiet sẽ ẩn output này.
pub fn blank_line() {
    if is_quiet() {
        return;
    }
    println!();
}

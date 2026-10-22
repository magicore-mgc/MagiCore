//! Startup banner presentation — trình bày banner khởi động.
//! This owner contains the banner text and the public printing function.
//! Owner này chứa nội dung banner và hàm in công khai.

use crate::quiet::is_quiet;
use console::style;
use std::fmt::Display;

pub(super) fn banner_display() -> impl Display {
    style(
        r#"
    ╔══════════════════════════════════════════╗
    ║            MagiCore               ║
    ║      Universal Package Manager           ║
    ╚══════════════════════════════════════════╝
    "#,
    )
    .cyan()
    .bold()
}

/// Print the MagiCore banner — in banner MagiCore.
/// Do nothing when quiet mode is active — không in khi bật quiet.
pub fn print_banner() {
    if is_quiet() {
        return;
    }
    println!("{}", banner_display());
}

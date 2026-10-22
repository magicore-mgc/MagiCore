//! Status-message tests — kiểm thử thông báo trạng thái.
//! Text builders are the same functions used by the terminal printers.
//! Các builder là hàm được printer terminal sử dụng trực tiếp.

use super::status::{error_line, hint_line, info_line, success_line, warning_line};

#[test]
fn status_lines_keep_their_distinct_prefixes_and_messages() {
    assert!(success_line("done").contains("✔"));
    assert!(success_line("done").contains("done"));
    assert!(error_line("failed").contains("✘"));
    assert!(error_line("failed").contains("failed"));
    assert!(warning_line("careful").contains("WARN:"));
    assert!(warning_line("careful").contains("careful"));
    assert!(info_line("ready").contains("ℹ"));
    assert!(info_line("ready").contains("ready"));
    assert!(hint_line("try again").contains("try again"));
}

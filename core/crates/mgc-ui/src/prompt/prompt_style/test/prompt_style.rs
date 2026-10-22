//! Shared prompt-style tests — kiểm thử style prompt dùng chung.
//! Pure formatting can be checked without an interactive terminal.
//! Có thể kiểm tra format thuần mà không mở terminal tương tác.

use super::style_prompt;

#[test]
fn prompt_style_keeps_marker_and_user_text() {
    let styled = style_prompt("Choose a runtime");
    assert!(styled.contains("◆"));
    assert!(styled.contains("Choose a runtime"));
}

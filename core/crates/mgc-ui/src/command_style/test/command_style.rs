//! Command style tests — kiểm thử định dạng tên lệnh.
//! These checks cover the returned presentation value.
//! Các kiểm tra này bao phủ giá trị trình bày được trả về.

use super::command_style::style_cmd;

#[test]
fn command_styling_keeps_original_command_text() {
    assert!(style_cmd("mgc init").contains("mgc init"));
}
